import { createContext, useContext, useState, useCallback } from "react";
import { X } from "lucide-react";

const ToastContext = createContext(undefined);

export function useToast() {
    const context = useContext(ToastContext);
    if (!context) {
        throw new Error("useToast must be used within a ToastProvider");
    }
    return context;
}

export function ToastProvider({ children }) {
    const [toasts, setToasts] = useState([]);

    const addToast = useCallback((message, type) => {
        const id = Date.now().toString();
        const newToast = { id, message, type };

        setToasts((prev) => [newToast, ...prev]); // Latest first

        // Auto remove after 5 seconds
        setTimeout(() => {
            removeToast(id);
        }, 5000);
    }, []);

    const removeToast = useCallback((id) => {
        setToasts((prev) => prev.filter((toast) => toast.id !== id));
    }, []);

    const addError = useCallback(
        (message) => addToast(message, "error"),
        [addToast]
    );
    const addSuccess = useCallback(
        (message) => addToast(message, "success"),
        [addToast]
    );
    const addInfo = useCallback(
        (message) => addToast(message, "info"),
        [addToast]
    );

    return (
        <ToastContext.Provider
            value={{
                toasts,
                addToast,
                removeToast,
                addError,
                addSuccess,
                addInfo,
            }}
        >
            {children}
            <ToastContainer toasts={toasts} onRemove={removeToast} />
        </ToastContext.Provider>
    );
}

function ToastContainer({ toasts, onRemove }) {
    return (
        <div className="fixed bottom-4 left-4 z-50 space-y-2">
            {toasts.map((toast, index) => (
                <div
                    key={toast.id}
                    className={`
            flex items-center gap-3 p-4 rounded-lg shadow-lg max-w-md
            transform transition-all duration-300 ease-in-out
            ${index > 0 ? "opacity-80" : "opacity-100"}
            ${toast.type === "error" ? "bg-red-600 text-white" : ""}
            ${toast.type === "success" ? "bg-green-600 text-white" : ""}
            ${toast.type === "info" ? "bg-blue-600 text-white" : ""}
            animate-slide-in
          `}
                    style={{
                        transform: `translateY(${index * -8}px)`,
                        zIndex: 50 - index,
                    }}
                >
                    <div className="flex-1 text-sm font-medium">
                        {toast.message}
                    </div>
                    <button
                        onClick={() => onRemove(toast.id)}
                        className="flex-shrink-0 p-1 hover:bg-black/20 rounded transition-colors"
                        aria-label="Close notification"
                    >
                        <X className="w-4 h-4" />
                    </button>
                </div>
            ))}
        </div>
    );
}
