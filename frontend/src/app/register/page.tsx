"use client"; 
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import Button from "../components/button";

export default function Register() {
    const router = useRouter();
    const [role, setRole] = useState<string | null>(null);
    const [formData, setFormData] = useState({ email: "", password: "" });
    const [message, setMessage] = useState("");

    // useEffect(() => {
    //     params.then(({ role }) => setRole(role)).catch((err) => console.error(err));
    // }, [params]);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        // if (!role) return; // Sécurité pour éviter un envoi avec `role` vide

        try {
            const response = await fetch(`http://127.0.0.1:8080/register`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(formData),
            });

            setMessage(response.ok ? "Inscription envoyée pour validation !" : "Erreur lors de l'inscription.");
        } catch (error) {
            console.error("Erreur:", error);
            setMessage("Erreur réseau");
        }
    };

    // if (!role) return <p>Chargement...</p>; // Afficher un état de chargement initial

    return (
        <div>
            <h2>Inscription</h2>
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
                <Button type="submit">S'inscrire</Button>
            </form>
            {message && <p>{message}</p>}

            <Button onClick={() => router.push("/login")}>
                Déjà inscrit ? Connectez-vous
            </Button>
        </div>
    );
}
