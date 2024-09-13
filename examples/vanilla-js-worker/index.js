import express from 'express';

const app = express();
// Serve static content from 'public' directory.
app.use(express.static('public'));
// Serve library as if in 'pkg' directory.
app.use('/pkg', express.static('../../lib-js/pkg'));
app.listen(3000);

console.log('Serving on http://localhost:3000/');
