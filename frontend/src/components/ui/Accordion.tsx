// components/Accordion.tsx
import { useState, ReactNode } from "react";

type AccordionProps = {
    title: string;
    children: ReactNode;
};

export const Accordion = ({ title, children }: AccordionProps) => {
    const [isOpen, setIsOpen] = useState(false);

    return (
        <div className="border rounded-lg overflow-hidden">
            <button
                onClick={() => setIsOpen(!isOpen)}
                className="w-full text-left px-5 py-3 bg-gray-100 hover:bg-gray-200 font-medium flex justify-between items-center"
            >
                {title}
                <span className="ml-2">{isOpen ? "▲" : "▼"}</span>
            </button>
            {isOpen && <div className="p-4 bg-white">{children}</div>}
        </div>
    );
};
