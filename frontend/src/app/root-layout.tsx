// Layout avec `"use client"` pour les effets et styles
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

    console.log(isAuthenticated);
    useEffect(() => {
        document.body.classList.add('bg-gray-100', 'h-screen');
    }, []);

    return (
        <html lang="en">
            <body>
                <header>
                    <nav>
                        <Button onClick={() => router.push("/")}>Home</Button>
                        {isAuthenticated ? (
                            <Button onClick={logout}>Logout</Button>
                        ) : (
                            <Button onClick={() => router.push("/login")}>Login</Button>
                        )}
                        <Button onClick={()=> router.push("/profile")}>Profile</Button>
                    </nav>
                </header>
                <main className="h-screen flex items-center justify-center gap-4 bg-gray-100">
                    <div className="w-full max-w-lg p-4">
                        {children}
                    </div>
                </main>
                <footer>
                    <p>© 2025 Mon Site Super Cool</p>
                </footer>
            </body>
        </html>
    )
}
