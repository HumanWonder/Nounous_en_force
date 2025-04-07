'use client';

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import Button from "./components/button";
import { useAuth } from "./hooks/useAuth";

export default function RootLayout({
    children,
}: {
    children: React.ReactNode
}) {
    const {isAuthenticated, logout} = useAuth();
    const router = useRouter();

    useEffect(() => {
        document.body.classList.add('h-screen');
    }, []);

    return (
        <html lang="en">
            <body>
                <header>
                    <nav className="flex justify-center gap-20 items-center p-4 bg-rose-100 shadow-md">
                        <Button onClick={() => router.push("/")}>Home</Button>
                        {isAuthenticated ? (
                            <Button onClick={logout}>Logout</Button>
                        ) : (
                            <Button onClick={() => router.push("/login")}>Login</Button>
                        )}
                        <Button onClick={()=> router.push("/profile")}>Profile</Button>
                    </nav>
                </header>
                <main className="h-screen flex items-center justify-center gap-4 bg-gray-50">
                    <div className="w-full p-4">
                        {children}
                    </div>
                </main>
                <footer className="text-center p-4 bg-rose-100">
                    <p>© 2025 Mon Site Super Cool</p>
                </footer>
            </body>
        </html>
    );
}
