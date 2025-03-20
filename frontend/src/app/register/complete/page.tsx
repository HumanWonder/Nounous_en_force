/* Choix form créche ou intervenant.e */
"use client";
import { useRouter } from "next/navigation";

export default function CompleteRegistration() {
    const router = useRouter();

    return (
        <div>
            <h2>Complétez votre inscription</h2>
            <br/>
            <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                onClick={() => router.push("complete/creche")}>
                Je suis un.e responsable de crèche
            </button>
            <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                onClick={() => router.push("complete/intervenant")}>
                Je suis un.e intervenant.e
            </button>
        </div>
    );
}