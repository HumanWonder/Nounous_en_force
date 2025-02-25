"use client";

import { useRouter } from "next/navigation";
import Button from "./components/button";

export default function Home() {
    const router = useRouter();

    return (
        <div className="h-screen">
            <h1>Bienvenue</h1>
            <Button onClick={() => router.push("/register/owner")}>
                Inscription Responsable de créche
            </Button>
            <Button onClick={() => router.push("/register/temp")}>
                Inscription Intervenant.e
            </Button>
        </div>
    );
}