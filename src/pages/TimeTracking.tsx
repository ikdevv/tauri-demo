import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

type TimerStatus = "idle" | "running" | "paused";

interface Break {
  id: number;
  session_id: number;
  start_time: string;
  end_time: string;
  duration: number;
}

interface TimerSession {
  id: number;
  start_time: string;
  end_time: string | null;
  duration: number;
  created_at: string;
}

interface SessionData {
  session: TimerSession;
  breaks: Break[];
}

function TimeTracking() {
  const [elapsed, setElapsed] = useState(0);
  const [status, setStatus] = useState<TimerStatus>("idle");
  const [sessions, setSessions] = useState<SessionData[]>([]);
  const [startTime, setStartTime] = useState<Date | null>(null);

  useEffect(() => {
    loadSessions();
  }, []);

  useEffect(() => {
    if (status !== "running") return;

    const interval = setInterval(() => {
      setElapsed((prev) => prev + 1);
    }, 1000);

    return () => clearInterval(interval);
  }, [status]);

  const loadSessions = async () => {
    try {
      const data = await invoke<SessionData[]>("get_sessions_with_breaks");
      setSessions(data);
    } catch (error) {
      console.error("Failed to load sessions:", error);
    }
  };

  const formatTime = (seconds: number) => {
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${String(hrs).padStart(2, "0")}:${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleString();
  };

  const handleStart = () => {
    setStartTime(new Date());
    setElapsed(0);
    setStatus("running");
  };

  const handlePause = () => {
    setStatus("paused");
  };

  const handleResume = () => {
    setStatus("running");
  };

  const handleStop = async () => {
    setStatus("idle");
    if (elapsed > 0 && startTime) {
      try {
        await invoke<number>("save_timer_session", {
          startTime: startTime.toISOString(),
          duration: elapsed,
        });

        await loadSessions();
      } catch (error) {
        console.error("Failed to save session:", error);
      }
    }
    setElapsed(0);
    setStartTime(null);
  };

  return (
    <main className="min-h-screen bg-gray-950 py-8 px-4">
      <div className="max-w-4xl mx-auto">
        {/* Timer Display */}
        <div className="w-full bg-gray-900 rounded-2xl shadow-xl p-8 mb-8">
          <h1 className="text-3xl font-bold text-white text-center mb-12">Time Tracker</h1>

          <div className="bg-gray-800 rounded-xl p-8 mb-8">
            <div className="text-6xl font-mono font-bold text-blue-400 text-center tracking-wider">
              {formatTime(elapsed)}
            </div>
          </div>

          {/* Controls */}
          <div className="flex flex-col gap-3">
            {status === "idle" && (
              <button
                onClick={handleStart}
                className="w-full py-3 bg-green-600 hover:bg-green-500 text-white font-semibold rounded-lg transition-colors"
              >
                Start
              </button>
            )}

            {status === "running" && (
              <>
                <button
                  onClick={handlePause}
                  className="w-full py-3 bg-yellow-600 hover:bg-yellow-500 text-white font-semibold rounded-lg transition-colors"
                >
                  Pause
                </button>
                <button
                  onClick={handleStop}
                  className="w-full py-3 bg-red-600 hover:bg-red-500 text-white font-semibold rounded-lg transition-colors"
                >
                  Stop
                </button>
              </>
            )}

            {status === "paused" && (
              <>
                <button
                  onClick={handleResume}
                  className="w-full py-3 bg-green-600 hover:bg-green-500 text-white font-semibold rounded-lg transition-colors"
                >
                  Resume
                </button>
                <button
                  onClick={handleStop}
                  className="w-full py-3 bg-red-600 hover:bg-red-500 text-white font-semibold rounded-lg transition-colors"
                >
                  Stop
                </button>
              </>
            )}
          </div>

        </div>

        {/* Session History */}
        <div className="w-full bg-gray-900 rounded-2xl shadow-xl p-8">
          <h2 className="text-2xl font-bold text-white mb-6">Session History</h2>

          {sessions.length === 0 ? (
            <p className="text-gray-400 text-center py-8">No sessions recorded yet</p>
          ) : (
            <div className="space-y-6 max-h-96 overflow-y-auto">
              {sessions.map((data) => (
                <div key={data.session.id} className="p-4 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors">
                  <div className="flex justify-between items-start mb-3">
                    <div>
                      <p className="text-gray-300 text-sm">{formatDate(data.session.start_time)}</p>
                      <p className="text-blue-400 font-mono font-semibold text-lg">{formatTime(data.session.duration)}</p>
                    </div>
                    {data.session.end_time && (
                      <div className="text-right text-sm text-gray-400">
                        <p>Ended: {new Date(data.session.end_time).toLocaleTimeString()}</p>
                      </div>
                    )}
                  </div>

                  {/* Breaks */}
                  {data.breaks.length > 0 && (
                    <div className="mt-3 pt-3 border-t border-gray-700">
                      <p className="text-gray-400 text-xs mb-2">Breaks ({data.breaks.length}):</p>
                      <div className="space-y-1">
                        {data.breaks.map((brk) => (
                          <div key={brk.id} className="text-xs text-gray-500 flex justify-between">
                            <span>{formatTime(brk.duration)}</span>
                            <span>{new Date(brk.start_time).toLocaleTimeString()} - {new Date(brk.end_time).toLocaleTimeString()}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </main>
  );
}

export default TimeTracking;
