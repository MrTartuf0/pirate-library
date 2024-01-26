// Import required packages
import bodyParser from 'body-parser';
import express, { Request, Response } from 'express';
import mongoose from 'mongoose';

// Set up Express
const app = express();
const PORT = 3000;

// Connect to MongoDB
mongoose.connect('mongodb://root:example@mongo:27017');

const db = mongoose.connection;
db.on('error', console.error.bind(console, 'MongoDB connection error:'));
db.once('open', () => console.log('Connected to MongoDB'));

const bookSchema = new mongoose.Schema({
  title: String,
  thumbnail: String,
  year: Number,
  language: String,
  isbn: Number,
  pages: Number,
  filetype: String,
  author: String,
  publisher: String,
  categories: [String],
  resource: String,
});

const Book = mongoose.model('Book', bookSchema);

// Middleware for parsing JSON requests
app.use(bodyParser.json());

// GET /search?q=atomic+habits
app.get('/search', async (req: Request, res: Response) => {
  try {
    const results = await Book.find({ title: new RegExp(req.query.q as string, 'i') });
    res.json({ results });
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

// GET /book/:isbn
app.get('/book/:isbn', async (req: Request, res: Response) => {
  try {
    const book = await Book.findOne({ isbn: req.params.isbn });
    if (book) {
      res.json(book);
    } else {
      res.status(404).json({ error: 'Book not found' });
    }
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

// POST /book
app.post('/book', async (req: Request, res: Response) => {
  try {
    const newBook = new Book(req.body);
    await newBook.save();
    res.status(201).json(newBook);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

// PUT /book/:isbn
app.put('/book/:isbn', async (req: Request, res: Response) => {
  try {
    const updatedBook = await Book.findOneAndUpdate({ isbn: req.params.isbn }, req.body, { new: true });
    if (updatedBook) {
      res.json(updatedBook);
    } else {
      res.status(404).json({ error: 'Book not found' });
    }
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

// DELETE /book/:isbn
app.delete('/book/:isbn', async (req: Request, res: Response) => {
  try {
    const deletedBook = await Book.findOneAndDelete({ isbn: req.params.isbn });
    if (deletedBook) {
      res.json(deletedBook);
    } else {
      res.status(404).json({ error: 'Book not found' });
    }
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Internal Server Error' });
  }
});

// Start the server
app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
