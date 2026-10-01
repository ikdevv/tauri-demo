import { Routes, Route, Link } from "react-router-dom";
import { Home } from "./pages/Home";
import { About } from "./pages/About";
import TimeTracking from "./pages/TimeTracking";

function App() {
  return (
    <div className="bg-gray-950 min-h-screen">
      <nav className="bg-gray-900 border-b border-gray-800">
        <div className="max-w-4xl mx-auto px-6 py-4 flex gap-6">
          <Link
            to="/"
            className="text-white font-semibold hover:text-blue-400 transition-colors"
          >
            Home
          </Link>
          <Link
            to="/about"
            className="text-white font-semibold hover:text-blue-400 transition-colors"
          >
            About
          </Link>
          <Link
            to="/time-tracking"
            className="text-white font-semibold hover:text-blue-400 transition-colors"
          >
            Time Tracking
          </Link>
        </div>
      </nav>

      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/about" element={<About />} />
        <Route path="/time-tracking" element={<TimeTracking />} />
      </Routes>
    </div>
  );
}

export default App;
