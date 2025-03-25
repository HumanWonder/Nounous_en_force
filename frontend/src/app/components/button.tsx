// components/Button.tsx
import React from 'react';

type ButtonProps = {
    onClick?: () => void;
    children: React.ReactNode;
    className?: string; // Pour ajouter des classes Tailwind supplémentaires si nécessaire
    type?: 'button' | 'submit' | 'reset'; // Pour changer le type du bouton
};

const Button: React.FC<ButtonProps> = ({ onClick, children, className = '', type = 'button' }) => {
    return (
        <button
            type={type}
            onClick={onClick}
            className={`text-black bg-gradient-to-r from-teal-100 to-cyan-300/80 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-cyan-300 dark:focus:ring-cyan-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2 ${className}`}
        >
            {children}
        </button>
    );
};

export default Button;
