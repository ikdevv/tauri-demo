export function About() {
  return (
    <main className="min-h-screen bg-gray-950 flex items-center justify-center">
      <div className="w-full max-w-md bg-gray-900 rounded-2xl shadow-xl p-8">
        <h1 className="text-3xl font-bold text-white text-center mb-6">
          About
        </h1>
        <div className="space-y-4 text-gray-300">
          <p>
            This is a Tauri demo application built with React and React Router.
          </p>
          <p>
            It demonstrates how to use React Router to navigate between different
            pages in a Tauri application.
          </p>
          <p className="text-sm text-gray-500">
            Click the navigation links at the top to switch between pages.
          </p>
        </div>
      </div>
    </main>
  );
}
