"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import Button from "../components/button";

export default function Login() {

    const router = useRouter();
    const [mounted, setMounted] = useState(false);

    const [formData, setFormData] = useState({ email: "", password: "" });
    const [message, setMessage] = useState("");

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        try {
            const response = await fetch("http://127.0.0.1:8080/login", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(formData),
            });
            const text = await response.text();  // Affiche la réponse brute
            // console.log("Réponse brute du serveur:", text);

            const data = JSON.parse(text); // Convertir en JSON
            // console.log("Réponse JSON du serveur:", data);

            if (response.ok) {
                setMessage("Connexion réussie !");
                console.log("REDIRECTION");
                setTimeout(() => router.push("/profile"), 2000);
            } else {
                if (data.message) {
                    //Message spécifique du serveur
                    setMessage(data.message);
                } else {
                    //Message générique
                    setMessage("Erreur survenue. Essayez à nouveau.");
                }
            }
        } catch (error) {
            console.error("Erreur:", error);
            //Erreur réseau ou interne au serveur
            setMessage("Erreur réseau. Vérifiez votre connexion et réessayez.");
        }
    };
    useEffect(() => {
        console.log(window.location.href);
        setMounted(true);
    }, []);

    if (!mounted) {
        // Affiche un contenu temporaire pendant le rendu SSR
        return null; // ou un chargement
    }

    return (
        <div>
            <h2>Connexion</h2>
            <form onSubmit={handleSubmit}>
                <input
                    type="email"
                    name="email"
                    placeholder="Email"
                    value={formData.email}
                    onChange={handleChange}
                    required
                />
                <input
                    type="password"
                    name="password"
                    placeholder="Mot de passe"
                    value={formData.password}
                    onChange={handleChange}
                    required
                />
                <Button type="submit">Se connecter</Button>
            </form>
            {message && <p>{message}</p>}
            <br />

            <br />
            <Button onClick={() => router.push("/register")}>
                Pas encore de compte ? Inscrivez-vous dès maintenant !
            </Button>
        </div>
    );
}
