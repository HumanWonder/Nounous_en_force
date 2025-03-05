'use client';
import { useRouter, useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";

export default function VerifyEmail() {
    const router = useRouter();
    const token = useSearchParams().get("token");
    const [message, setMessage] = useState("Vérification en cours...");

    useEffect(() => {
        if (!token) return;

        console.log("Token récupéré :", token); // Debug

        fetch("http://localhost:8080/verify_email", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ token }),
        })
        .then(async res => {
            if (!res.ok) {
                throw new Error(`HTTP error! Status: ${res.status}`);
            }
            const contentType = res.headers.get("content-type");
            if (!contentType || !contentType.includes("application/json")) {
                throw new Error("La réponse n'est pas du JSON valide");
            }
            return res.json();
        })
        .then(data => {
            console.log(data);
            if (data.success) {
                setMessage("Votre email a été vérifié avec succès !");
                setTimeout(() => router.push("/login"), 3000); // Redirection après 2s
            } else {
                setMessage("Échec de la vérification de l'email.");
                setTimeout(() => router.push("/error"), 2000);
            }
        })
        .catch(err => {
            console.error("Erreur de requête : ", err);
            setMessage("Une erreur est survenue.");
        });
    }, [token]);

    return <p>{message}</p>;
}
