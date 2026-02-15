import { Link } from 'react-router';

function NotFoundView() {
  return (
    <alert>
      <h2>Not found</h2>
      <p>The page you are looking for does not exist.</p>
      <Link to="/">Go back home</Link>
    </alert>
  );
}

export default NotFoundView;
