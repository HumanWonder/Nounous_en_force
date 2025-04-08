"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../hooks/useAuth";

import type { FullProfileData } from "../types/user";

export default function Profile() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();
    const [profileData, setProfileData] = useState<FullProfileData | null>(null);
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
            console.log("Token:", token);
            console.log("isAuthenticated:", isAuthenticated);

            try {
                const response = await fetch("http://127.0.0.1:8080/profile", {
                    method: "GET",
                    headers: {
                        "Authorization": `Bearer ${token}`,  // Ajouter le token dans l'en-tête
                    },
                    credentials: "include",
                });

                const data = await response.json();
                console.log("data :", data);

                if (response.ok) {

                    setProfileData({
                        user: {
                            email: data.email,
                            role: data.role,

                        },
                        ...(data.role === "temp" && data.temp && { temp_data: data.temp }),
                        ...(data.role === "owner" && data.owner && { owner_data: data.owner }),
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
        console.log("USERDATA mis à jour :", profileData);
    }, [profileData]);


    return (
        <div>
            {message && <p>{message}</p>}
            {profileData ? (
                <div>
                    <h2>Profil de l'utilisateur</h2>
                    <br />
                    <p>Email : {profileData.user.email}</p>
                    <p>Rôle : {profileData.user.role}</p>
                    <br />
                    {profileData.user.role === "pending" && (
                        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                            onClick={() => router.push("register/complete/")}>
                            Compléter mon inscription
                        </button>
                    )}

                    {profileData.user.role === "admin" && (
                        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                            onClick={() => router.push("/admin")}>
                            Page Admin
                        </button>
                    )}
                    {/* Affiche d'autres données utilisateur ici */}
                    {profileData.user.role === "temp" && profileData.temp_data ? (
                        <>
                            <h3>Informations intérimaires</h3>
                            <br />
                            <p>Nom complet: {profileData.temp_data?.temp_info.full_name}</p>
                            <p>Adresse: {profileData.temp_data?.temp_info.address}</p>
                            <p>Téléphone: {profileData.temp_data?.temp_info.phone}</p>
                            {profileData.temp_data?.temp_info.birth_date && <p>Date de naissance: {profileData.temp_data?.temp_info.birth_date}</p>}
                            <p>Permis de conduire: {profileData.temp_data?.temp_info.driver_license ? "Oui" : "Non"}</p>
                            <p>Moyen de transport: {profileData.temp_data?.temp_info.transport}</p>
                            {profileData.temp_data?.temp_info.motivation && <p>Motivation: {profileData.temp_data?.temp_info.motivation}</p>}
                            <p>Casier judiciaire: {profileData.temp_data?.temp_info.judicial_record}</p>
                            {/* Disponibilités */}
                            <h4 className="text-lg font-medium mt-4">Disponibilités</h4>
                            {profileData.temp_data.availabilities.map((a, index) => (

                                <div key={index} className="ml-4 mb-2">
                                    <p>Périodes disponibles: {a.available_periods}</p>
                                    <p>Horaires: {a.work_hours}</p>
                                    <p>Zones préférées: {a.preferred_locations}</p>
                                    <p>Temps de trajet max: {a.max_travel_time}</p>
                                </div>
                            ))}

                            {/* Conditions de travail */}
                            <h4 className="text-lg font-medium mt-4">Conditions de travail</h4>
                            {profileData.temp_data?.conditions?.map((c, index) => (
                                <div key={index} className="ml-4 mb-2">
                                    <p>Taux horaire: {c.hourly_rate}€</p>
                                    <p>Types de contrat: {c.contract_types}</p>
                                    <p>Auto-entrepreneur: {c.self_employment ? "Oui" : "Non"}</p>
                                </div>
                            ))}

                            {/* Diplômes */}
                            <h4 className="text-lg font-medium mt-4">Diplômes</h4>
                            {profileData.temp_data?.documents?.map((d, index) => (
                                <div key={index} className="ml-4 mb-2">
                                    <p>Diplôme: {d.diploma_name}</p>
                                    <p>Autres certifications: {d.other_certifications}</p>
                                    <p>Année d'obtention: {d.year_obtained}</p>
                                    <p>Établissement: {d.institution}</p>
                                </div>
                            ))}

                            {/* Expériences */}
                            <h4 className="text-lg font-medium mt-4">Expériences professionnelles</h4>
                            {profileData.temp_data?.experiences?.map((e, index) => (
                                <div key={index} className="ml-4 mb-2">
                                    <p>Total d'expérience: {e.total_experience}</p>
                                    <p>Postes précédents: {e.previous_jobs}</p>
                                    <p>Types de structures: {e.structure_types}</p>
                                    <p>Tâches réalisées: {e.tasks}</p>
                                </div>
                            ))}
                        </>
                    ) : null}
                </div>
            ) : (
                <p>Chargement...</p>
            )}
        </div>
    );
}
