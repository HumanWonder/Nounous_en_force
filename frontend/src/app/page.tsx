"use client";

import { useRouter } from "next/navigation";

export default function Home() {
    const router = useRouter();

    return (
        <div>
            <h1>Bienvenue</h1>
            <button onClick={() => router.push("/register/owner")}>
                Inscription Responsable de créche
            </button>
            <button onClick={() => router.push("/register/temp")}>
                Inscription Intervenant.e
            </button>
        </div>
    );
}