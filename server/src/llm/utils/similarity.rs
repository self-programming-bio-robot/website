pub fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let magnitude_vec1: f64 = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude_vec2: f64 = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    dot_product / (magnitude_vec1 * magnitude_vec2)
}