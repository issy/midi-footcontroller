import { Outlet } from 'react-router';

function AppLayout() {
  return (
    <div className="min-h-screen bg-gray-950 text-white">
      <Outlet />
    </div>
  );
}

export default AppLayout;
