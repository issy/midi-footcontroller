import { Link } from 'react-router';

function NotFoundView() {
  return (
    <div className="min-h-screen flex items-center justify-center px-6">
      <div className="max-w-xl w-full text-center relative">
        {/* Glow background */}
        <div className="absolute inset-0 -z-10 blur-3xl opacity-30 bg-gradient-to-tr from-indigo-500 via-purple-500 to-pink-500 rounded-full" />

        {/* Content */}
        <div className="transition-all duration-500 ease-out translate-y-0 opacity-100">
          <h1 className="text-7xl md:text-8xl font-bold tracking-tight mb-4 bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">
            404
          </h1>

          <h2 className="text-2xl md:text-3xl font-semibold mb-3">Page not found</h2>

          <p className="text-gray-400 mb-8">The page you’re looking for doesn’t exist or has been moved.</p>

          {/* Actions */}
          <div className="flex items-center justify-center gap-4">
            <Link
              to="/"
              className="px-6 py-3 rounded-2xl bg-indigo-600 hover:bg-indigo-500 transition-all duration-200 font-medium shadow-lg hover:scale-[1.03] active:scale-[0.97]"
            >
              Go home
            </Link>{' '}
          </div>
        </div>
      </div>
    </div>
  );
}

export default NotFoundView;
