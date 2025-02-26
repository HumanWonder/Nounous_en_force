// Layout avec `"use client"` pour les effets et styles
'use client';

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import Button from "./components/button";

export default function RootLayout({
    children,
}: {
    children: React.ReactNode
}) {
    const router = useRouter();
    useEffect(() => {
        document.body.classList.add('bg-gray-100', 'h-screen');
    }, []);

    return (
        <html lang="en">
            <body>
                <header>
                    <nav>
                        <Button onClick={() => router.push("/")}>Home</Button>
                        <Button onClick={()=> router.push("/login")}>Connexion</Button>
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
