import { Link } from 'react-router';

function HomeView() {
  return (
    <div>
      <h2>This is the homepage</h2>
      <p>This is the homepage content</p>
      <Link to="/device">Manage your device</Link>
    </div>
  );
}

export default HomeView;
