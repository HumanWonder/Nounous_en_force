"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../hooks/useAuth";


// Déclare un type pour les données de l'utilisateur, sinon TypeScript panique en voyant null
type UserData = {
    email: string;
    role: string;
    temp?: TempData;    //Champs optionnels selon rôle
};

type TempData = {
    full_name: string;
    address: string;
    phone: string;
    birth_date?: string;
    driver_license: boolean;
    transport: string;
    motivation?: string;
    judicial_record: string;
};

export default function Profile() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();
    const [userData, setUserData] = useState<UserData | null>(null);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(true); // Ajout d'un état de chargement

    useEffect(() => {
        const fetchUserProfile = async () => {
            // // Récupérer le token depuis les cookies (ajuster selon outil de gestion de cookies)
            console.log("Token in profile : ", token)
            if (!token || !isAuthenticated) {
                setMessage("Token inexistant, connexion non authentifiée");
                // setTimeout(() => router.push("/login"), 5000); // Redirige vers la page de login si aucun token
                return;
            }

            try {
                const response = await fetch("http://127.0.0.1:8080/profile", {
                    method: "GET",
                    headers: {
                        "Authorization": `Bearer ${token}`,  // Ajouter le token dans l'en-tête
                    },
                    credentials: "include",
                });

                const data = await response.json();
                console.log("data :",data);

                if (response.ok) {
                    setUserData({
                        email: data.user.email,
                        role: data.user.role,
                        temp: data.temp ?? undefined, // Ajoute `temp` s'il existe
                    });
                } else {
                    setMessage(data.message || "Erreur de récupération des données. Veuillez vous reconnecter.");

                    // setTimeout(() => router.push("/login"), 5000); // Redirige vers la page de login si le token est invalide
                }
            } catch (error) {
                console.error("Erreur lors de la récupération du profil:", error);
                setMessage("Erreur réseau. Vérifiez votre connexion.");
            } finally {
                setLoading(false); // Marque le chargement comme terminé
            }
        };

        if (token && isAuthenticated) {
            fetchUserProfile();
        } else {
            setLoading(false); // Marque aussi comme terminé si le token n'est pas présent
        }
    }, [isAuthenticated, token, router]);

    useEffect(() => {
        console.log("USERDATA mis à jour :", userData);
    }, [userData]);
    

    if (loading) {
        return <p>Chargement des données...</p>; // Affiche un message de chargement
    }

    return (
        <div>
            {message && <p>{message}</p>}
            {userData ? (
                <div>
                    <h2>Profil de l'utilisateur</h2>
                    <br />
                    <p>Email : {userData.email}</p>
                    <p>Rôle : {userData.role}</p>
                    <br/>
                    {userData.role === "pending" && (
                        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                            onClick={() => router.push("register/complete/")}>
                            Compléter mon inscription
                        </button>
                    )}

                    {userData.role === "admin" && (
                        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                            onClick={() => router.push("/admin")}>
                            Page Admin
                        </button>
                    )}
                    {/* Affiche d'autres données utilisateur ici */}
                    {userData.role === "temp" && userData.temp ? (
                        <>
                            <h3>Informations intérimaires</h3>
                            <br/>
                            <p>Nom complet: {userData.temp.full_name}</p>
                            <p>Adresse: {userData.temp.address}</p>
                            <p>Téléphone: {userData.temp.phone}</p>
                            {userData.temp.birth_date && <p>Date de naissance: {userData.temp.birth_date}</p>}
                            <p>Permis de conduire: {userData.temp.driver_license ? "Oui" : "Non"}</p>
                            <p>Moyen de transport: {userData.temp.transport}</p>
                            {userData.temp.motivation && <p>Motivation: {userData.temp.motivation}</p>}
                            <p>Casier judiciaire: {userData.temp.judicial_record}</p>
                        </>
                    ) : null }
                </div>
            ) : (
                <p>Veuillez vous connecter</p>
            )}
        </div>
    );
}
