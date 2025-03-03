'use client';
import { useRouter, useSearchParams } from "next/navigation";
import { NextRequest } from "next/server";
import { useEffect, useState } from "react";

export default function VerifyEmail() {
    const router = useRouter();
    const token = useSearchParams().get("token");
    const [message, setMessage] = useState("Vérification en cours...");

    useEffect(() => {
        if (!token) return;

        console.log("Token récupéré :", token); // Debug

        fetch("http://127.0.0.1:8080/verify_email", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ token }),
        })
        .then(res => res.json())
        .then(data => {
            if (data.success) {
                setMessage("Votre email a été vérifié avec succès !");
                setTimeout(() => router.push("/"), 2000); // Redirection après 2s
            } else {
                setMessage("Échec de la vérification de l'email.");
                setTimeout(() => router.push("/error"), 2000);
            }
        })
        .catch(err => {
            console.error("Erreur de requête :", err);
            setMessage("Une erreur est survenue.");
        });
    }, [token]);

    return <p>{message}</p>;
}
