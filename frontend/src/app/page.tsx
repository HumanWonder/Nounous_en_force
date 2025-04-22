"use client";

import { useRouter } from "next/navigation";
import Button from "./components/button";

export default function Home() {
    const router = useRouter();

    return (
        <div className="text-center bg-rose-200/50 p-8 rounded-lg shadow-lg">
            <h1 className="text-4xl text-red-300 font-semibold mb-4">Bienvenue sur notre plateforme</h1>
            <p className="text-lg mb-6 ">Mettez en relation les responsables de crèches avec des intervenants qualifiés pour un remplacement ou une mission. Simple, rapide et sécurisé.</p>
            
            <div className="flex justify-center gap-4">
                <Button onClick={() => router.push("/login")}>Connectez-vous</Button>
            </div>
        </div>
    );
}