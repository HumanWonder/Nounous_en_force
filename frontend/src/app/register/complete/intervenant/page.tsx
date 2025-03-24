/*Form pour les intervenant.e.s */
"use client";
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '../../../hooks/useAuth';

export default function IntervenantRegister() {

    const router = useRouter();
    const { token, isAuthenticated } = useAuth();

    const [formData, setFormData] = useState({
        full_name: '',
        address: '',
        phone: '',
        birth_date: '',
        driver_license: false,
        transport: '',
        motivation: '',
        judicial_record: '',
    });

    const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setFormData({ ...formData, [name]: value });
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        console.log("FormData : ", formData);
        console.log("Token : ", token);
        if (!token || !isAuthenticated) {
            alert("Token inexistant, connexion non authentifiée, redirection vers login");
            // setTimeout(() => router.push("/login"), 5000); // Redirige vers la page de login si aucun token
            return;
        }
        try {
            const response = await fetch("http://127.0.0.1:8080/register/temp", {
                method: "POST",
                //pour envoyer les cookies
                // credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${token}`  // Ajouter explicitement le token dans l'en-tête
                },
                body: JSON.stringify(formData),
            });
            console.log("response.text ::",response.text());
            const data = await response.json();
            console.log("DATA SENT : ",data);
            alert("Inscription enregistrée, en attente de validation par un administrateur.");
            // Rediriger l'utilisateur vers la page d'accueil
            router.push("/");
        } catch (error) {
            console.error("Erreur lors de l'envoi du formulaire :", error);
        }
    };
// A debug : formulaire type et réponse du serveur

    return (
        <form onSubmit={handleSubmit}>
            <input
                type="text"
                name="full_name"
                value={formData.full_name}
                onChange={handleChange}
                placeholder="Nom complet"
            />
            <input
                type="text"
                name="address"
                value={formData.address}
                onChange={handleChange}
                placeholder="Adresse"
            />
            <input
                type="text"
                name="phone"
                value={formData.phone}
                onChange={handleChange}
                placeholder="Téléphone"
            />
            <input
                type="date"
                name="birth_date"
                value={formData.birth_date}
                onChange={handleChange}
            />
            <br/>
            <input
                type="checkbox"
                name="driver_license"
                checked={formData.driver_license}
                onChange={() => setFormData({ ...formData, driver_license: !formData.driver_license })}
            />
            <br/>
            <input
                type="text"
                name="transport"
                value={formData.transport}
                onChange={handleChange}
                placeholder="Mode de transport"
            />
            <textarea
                name="motivation"
                value={formData.motivation}
                onChange={handleChange}
                placeholder="Motivation"
            />
            <input
                type="text"
                name="judicial_record"
                value={formData.judicial_record}
                onChange={handleChange}
                placeholder="Casier judiciaire"
            />
            <button type="submit">Envoyer</button>
        </form>
    );
}