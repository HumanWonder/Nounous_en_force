"use client"; //useState fonctionne uniquement côté client

import { useState } from "react";

export default function Form() {
    const [formData, setFormData] = useState({ email: "", password: "" });
    const [message, setMessage] = useState("");

    const handleChange = (e) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e) => {
        e.preventDefault();

        try {
            const response = await fetch("http://127.0.0.1:8080/register", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(formData),
            });

            if (response.ok) {
                setMessage("Inscription envoyée pour validation !");
            } else {
                setMessage("Erreur lors de l'inscription.");
            }
        } catch (error) {
            console.error("Erreur:", error);
            setMessage("Erreur réseau");
        }
    };

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
                <button type="submit">S'inscrire</button>
            </form>
            {message && <p>{message}</p>}
        </div>
    );
}
