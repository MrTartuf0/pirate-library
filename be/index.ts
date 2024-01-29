import bodyParser from 'body-parser';
import express, { Request, Response } from 'express';
import jwt from 'jsonwebtoken';
import mongoose from 'mongoose';
import multer from 'multer';

const app = express();
const PORT = 3000;
app.use(bodyParser.json());

mongoose.connect('mongodb://root:example@mongo:27017');

const db = mongoose.connection;
db.on('error', console.error.bind(console, 'MongoDB connection error:'));
db.once('open', () => console.log('Connected to MongoDB'));

interface UploadedFile {
  fieldname: string;
  originalname: string;
  encoding: string;
  mimetype: string;
  destination: string;
  filename: string;
  path: string;
  size: number;
}

const userSchema = new mongoose.Schema({
  email: { type: String, unique: true },
  password: String,
});

const bookSchema = new mongoose.Schema({
  isbn: { type: String, unique: true },
  title: String,
  added: { type: Date, default: Date.now },
  user: { type: mongoose.Schema.Types.ObjectId, ref: 'User' },
  url_link: String,
});

const User = mongoose.model('User', userSchema);
const Book = mongoose.model('Book', bookSchema);

const verifyToken = (req: Request, res: Response, next: any) => {
  const token = req.headers.authorization?.split(' ')[1];
  if (!token) return res.status(403).json({ error: 'No token provided' });

  jwt.verify(token, 'secret_key', (err: any, decoded: any) => {
    if (err) return res.status(401).json({ error: 'Failed to authenticate token' });
    req.userId = decoded.userId;
    next();
  });
};

const storage = multer.diskStorage({
  destination: function (req, file, cb) {
    cb(null, './rusted_files/web');
  },
  filename: function (req, file, cb) {
    cb(null, file.originalname);
  }
});

const upload = multer({ storage: storage });

app.post('/upload-book', upload.single('book'), async (req: Request, res: Response) => {
  try {
    const { isbn } = req.body;
    
    const token = req.headers.authorization?.split(' ')[1];
    if (!token) return res.status(403).json({ error: 'No token provided' });

    const decoded: any = jwt.verify(token, 'secret_key');
    const userId = decoded.userId;
    const uploadedFile = req.file as UploadedFile | undefined;

    if (!uploadedFile) {
      return res.status(400).json({ error: 'No file uploaded' });
    }

    const filename = uploadedFile.originalname;

    const newBook = new Book({
      isbn,
      title: filename,
      user: userId,
      url_link: "http://localhost:8000/" + filename,
    });

    await newBook.save();

    res.status(200).json({ message: 'Book uploaded successfully' });
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});


app.post('/login', async (req: Request, res: Response) => {
  const { email, password } = req.body;

  try {
    const user = await User.findOne({ email });
    if (!user) {
      return res.status(401).json({ error: 'Invalid email or password' });
    }

    const validPassword = await password === user.password;
    if (!validPassword) {
      return res.status(401).json({ error: 'Invalid email or password' });
    }

    const token = jwt.sign({ userId: user._id }, 'secret_key', { expiresIn: '1h' });
    res.json({ token });
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

app.post('/create', async (req: Request, res: Response) => {
  const { email, password } = req.body;

  try {
    const existingUser = await User.findOne({ email });
    if (existingUser) {
      return res.status(400).json({ error: 'User already exists' });
    }

    const newUser = new User({ email, password: password });
    await newUser.save();

    res.status(201).json({ message: 'User created successfully' });
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

app.get('/search-by-isbn/:isbn', async (req: Request, res: Response) => {
  const isbn = req.params.isbn;

  try {
    const book = await Book.findOne({ isbn });
    if (!book) {
      return res.status(404).json({ error: 'Book not found' });
    }
    res.json(book);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

app.get('/search-by-name/:name', async (req: Request, res: Response) => {
  const name = req.params.name;

  try {
    const books = await Book.find({ title: { $regex: new RegExp(name, 'i') } });
    if (books.length === 0) {
      return res.status(404).json({ error: 'No books found' });
    }
    res.json(books);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

app.get('/books', async (req: Request, res: Response) => {
  try {
    const page: number = req.query.page ? parseInt(req.query.page as string) : 1;
    const limit: number = 25;
    const skip: number = (page - 1) * limit;

    const books = await Book.find().skip(skip).limit(limit);

    res.json(books);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
