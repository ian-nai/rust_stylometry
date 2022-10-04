use unicode_segmentation::UnicodeSegmentation;
use plotters::prelude::*;

use std::fs;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::collections::HashMap;


pub fn stopword_filter(mut vec: Vec<String>, lang: &str) ->  Vec<String>{
    let s_words = stop_words::get_nltk(lang);
    for w in s_words{
            vec.retain(|word| *word != w);
        }
    vec

}
pub fn word_list_from_string(string_to_analyze: &str) -> Vec<String> {
    let mut word_list: Vec<String> = Vec::new();
    let w = string_to_analyze.unicode_words();
    for q in w {
        word_list.push(q.to_string());
    }
    word_list
}

pub fn word_list_from_file(filename: &str) -> Vec<String> {
    let string_from_file = fs::read_to_string(filename).expect("Unable to read file");
    let mut word_list: Vec<String> = Vec::new();
    let w = string_from_file.unicode_words();
    for q in w {
        word_list.push(q.to_string());
    }
    word_list
}

pub fn unique_lengths(vec: Vec<i32>) -> Vec<i32> {
    let mut unique_wordlengths = vec;
    unique_wordlengths.sort_unstable();
    unique_wordlengths.dedup();
    unique_wordlengths
}

pub fn get_wordlengths(text: &str) -> Vec<i32> {
    let text_word_list = word_list_from_file(text);
    let mut word_length_list: Vec<i32> = Vec::new();
    for w in text_word_list {
        word_length_list.push(w.len() as i32);
    }
    word_length_list
}

pub fn get_wordlengths_str(text: &str) -> Vec<i32> {
    let text_word_list = word_list_from_string(text);
    let mut word_length_list: Vec<i32> = Vec::new();
    for w in text_word_list {
        word_length_list.push(w.len() as i32);
    }
    word_length_list
}

pub fn create_length_vec(text: &str) -> Vec<i32> {
    let word_lengths = get_wordlengths(text);
    let unique_wordlengths = unique_lengths(word_lengths);
    unique_wordlengths
}

pub fn unique_counting(text: &str) -> Vec<i32> {
    let unique_counts = create_length_vec(text);
    unique_counts
}
pub fn get_total_wordcounts(text: &str) -> Vec<i32>{
    let word_lengths = get_wordlengths(text);
    let unique_counts = create_length_vec(text);

    let mut total_counts: Vec<i32> = Vec::new();
    for val in unique_counts {
        if word_lengths.contains(&val) {
            let count = word_lengths.iter().filter(|&n| *n == val).count() as i32;
            total_counts.push(count);
        }
    }
    total_counts
}

pub fn unique_and_total(text: &str) {
    let second_count = unique_counting(text);
    let total_wordcounts = get_total_wordcounts(text);
    zip_vecs(second_count, total_wordcounts);
}

pub fn scatterplot_string(text: &str) {
    let second_count = unique_counting(text);
    let total_wordcounts = get_total_wordcounts(text);
    zip_vecs(second_count, total_wordcounts);
}

pub fn scatterplot(file: &str) {
    let second_count = unique_counting(file);
    let total_wordcounts = get_total_wordcounts(file);
    zip_vecs(second_count, total_wordcounts);
}

pub fn zip_vecs(unique_counts: Vec<i32>, total_counts: Vec<i32>) {
    let combined_vector = unique_counts.into_iter().zip(total_counts).collect::<Vec<_>>();
    graph_scatter(combined_vector);
}

pub fn graph_scatter(combined_vector: Vec<(i32, i32)>) {
    let root_area = BitMapBackend::new("src/mendenhall_graph.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Word Length Scatterplot", ("sans-serif", 40))
        .build_cartesian_2d(1..20, 1..20)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
       combined_vector.iter().map(|point| Circle::new(*point, 4.0_f64, &BLUE)),
   ).unwrap();

}

pub fn mendenhall_file(file: &str) {
    let mut line_graph = get_wordlengths(file);
    line_graph.sort();

    let root_area = BitMapBackend::new("src/mendenhall_graph.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Word Length Graph", ("sans-serif", 40))
        .build_cartesian_2d(1..line_graph.len(), 1..20)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..).zip(line_graph.iter()).map(|(idx, y)| {(idx, *y)}),&BLUE)
    ).unwrap();
}

pub fn mendenhall_string(string: &str) {
    let mut line_graph = get_wordlengths_str(string);
    line_graph.sort();

    let root_area = BitMapBackend::new("src/mendenhall_graph.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Word Length Graph", ("sans-serif", 40))
        .build_cartesian_2d(1..line_graph.len(), 1..20)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..).zip(line_graph.iter()).map(|(idx, y)| {(idx, *y)}),&BLUE)
    ).unwrap();
}

pub fn combine_vecs_from_files(text1: &str, text2: &str) -> Vec<String> {
    let word_list1 = word_list_from_file(text1);
    let word_list2 = word_list_from_file(text2);

    let mut combined_documents: Vec<String> = Vec::new();
    combined_documents.extend(word_list1);
    combined_documents.extend(word_list2);
    combined_documents

}

pub fn get_freq(vec: &Vec<String>) -> BTreeMap<String, usize>  {
    let mut result: BTreeMap<String, usize> = BTreeMap::new();
    for key in vec {
        let val = result.entry((&key).to_string()).or_insert(0);
        *val += 1;
    }
    result
}

pub fn convert_map(map: BTreeMap<String, usize>) -> Vec<(String, usize)> {
    let mut v = Vec::from_iter(map);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    v
}

pub fn get_freq_hash(vec: Vec<String>) -> HashMap<String, usize>  {
    let mut result: HashMap<String, usize> = HashMap::new();
    for key in vec {
        let val = result.entry(key).or_insert(0);
        *val += 1;
    }

    result
}

pub fn kilgariff(text1: &str, text2: &str, num_words: usize, lang: &str) {
    let combined_vec = combine_vecs_from_files(text1, text2);
    let _combined_vec_length = combine_vecs_from_files(text1, text2).len() as f64;
    let no_stopwords_combined_vec = stopword_filter(combined_vec, lang);
    let _text1_length = word_list_from_file(text1).len() as f64;
    let _text2_length = word_list_from_file(text2).len() as f64;

    let word_freqs_map = get_freq(&no_stopwords_combined_vec);
    let _sorted_word_freqs = convert_map(word_freqs_map);

    let text1_vec = word_list_from_file(text1);
    let text2_vec = word_list_from_file(text2);

    let text1_freqs = get_freq(&text1_vec);
    let text2_freqs = get_freq(&text2_vec);

    let sorted_text1_freqs = convert_map(text1_freqs);
    let sorted_text2_freqs = convert_map(text2_freqs);

    let chosen_words = &no_stopwords_combined_vec[0..num_words];

    let text1_ind_count = compute_ind_count(sorted_text1_freqs, chosen_words);
    let text2_ind_count = compute_ind_count(sorted_text2_freqs, chosen_words);

    compute_joint_count(chosen_words, text1_ind_count, text2_ind_count);
}

pub fn compute_ind_count(vec1: Vec<(String, usize)>, chosen_words: &[String]) ->  Vec<usize>{

        let mut text1_occurrences: Vec<&usize> = Vec::new();

        for word in chosen_words {
            for (s,c) in &vec1 {
                if &word.as_str() == &s.as_str() {
                    text1_occurrences.push(&c);
                }
            }
        }

        let mut ind_counts: Vec<usize> = Vec::new();
        for s in text1_occurrences {
            ind_counts.push(*s);
        }
        ind_counts
    }

pub fn compute_joint_count(chosen_words: &[String], ind_count_1: Vec<usize>, ind_count_2: Vec<usize>){
    let mut joint_count_vec: Vec<usize> = Vec::new();
    let mut counter = 0;
    if ind_count_1.is_empty() {
        if ind_count_2.is_empty() {
            for _word in chosen_words {
                let joint_count = 0 + 0;
                joint_count_vec.push(joint_count);
                counter += 1;
            }
        }
        else {
            for _word in chosen_words {
                let joint_count = 0 + ind_count_2[counter];
                joint_count_vec.push(joint_count);
                counter += 1;
        }
    }
}
    else {
        if ind_count_2.is_empty() {
            for _word in chosen_words {
                let joint_count = ind_count_1[counter] + 0;
                joint_count_vec.push(joint_count);
                counter += 1;
            }
        }
        else {
            for _word in chosen_words {
                let joint_count = ind_count_1[counter] + ind_count_2[counter];
                joint_count_vec.push(joint_count);
                counter += 1;
            }
        }
    }
    kilgariff_chi_squared(chosen_words, ind_count_1, ind_count_2, joint_count_vec);

}

pub fn kilgariff_chi_squared(chosen_words: &[String], ind_count_1: Vec<usize>, ind_count_2: Vec<usize>, joint_count: Vec<usize>) {
    let counter = 0;
    let mut chisquared = 0 as f64;
    let _expected_text1_count = 0 as f64;
    let _expected_text2_count = 0 as f64;
    for _word in chosen_words {
        if ind_count_1.is_empty() {
            if ind_count_2.is_empty() {
                let expected_text1_count = joint_count[counter] as f64 * (0 as f64 / joint_count[counter] as f64);
                let expected_text2_count = joint_count[counter] as f64 * (0 as f64 / joint_count[counter] as f64);
                chisquared += (0 as f64 - expected_text1_count) * (0 as f64 - expected_text1_count / expected_text1_count);
                chisquared += (0 as f64 - expected_text2_count) * (0 as f64 - expected_text2_count / expected_text2_count);
                }
            else {
                let expected_text1_count = joint_count[counter] as f64 * (ind_count_1[counter] as f64 / joint_count[counter] as f64);
                let expected_text2_count = joint_count[counter] as f64 * (0 as f64 / joint_count[counter] as f64);
                chisquared += (ind_count_1[counter] as f64 - expected_text1_count) * (ind_count_1[counter] as f64 - expected_text1_count / expected_text1_count);
                chisquared += (0 as f64 - expected_text2_count) * (0 as f64 - expected_text2_count / expected_text2_count);
            }
        }
        else {
            if ind_count_2.is_empty() {
                let expected_text1_count = joint_count[counter] as f64 * (ind_count_1[counter] as f64 / joint_count[counter] as f64);
                let expected_text2_count = joint_count[counter] as f64 * (0 as f64 / joint_count[counter] as f64);
                chisquared += (ind_count_1[counter] as f64 - expected_text1_count) * (ind_count_1[counter] as f64 - expected_text1_count / expected_text1_count);
                chisquared += (0 as f64 - expected_text2_count) * (0 as f64 - expected_text2_count / expected_text2_count);
                }
            else {
                let expected_text1_count = joint_count[counter] as f64 * (ind_count_1[counter] as f64 / joint_count[counter] as f64);
                let expected_text2_count = joint_count[counter] as f64 * (ind_count_2[counter] as f64 / joint_count[counter] as f64);
                chisquared += (ind_count_1[counter] as f64 - expected_text1_count) * (ind_count_1[counter] as f64 - expected_text1_count / expected_text1_count);
                chisquared += (ind_count_2[counter] as f64 - expected_text2_count) * (ind_count_2[counter] as f64 - expected_text2_count / expected_text2_count);
            }
        }
    }

    println!("{}", chisquared);
}

pub fn hapax_single(file1: &str) -> Vec<String> {
    let mut file1_words = word_list_from_file(file1);

    file1_words.sort_unstable();
    file1_words.dedup();

    file1_words
}

pub fn hapax(file1: &str, file2: &str) -> Vec<String> {
    let mut file1_words = word_list_from_file(file1);
    let mut file2_words = word_list_from_file(file2);

    file1_words.sort_unstable();
    file1_words.dedup();

    file2_words.sort_unstable();
    file2_words.dedup();

    let file1_hash = get_freq_hash(file1_words);
    let file2_hash = get_freq_hash(file2_words);


     let mut hapax_list: Vec<String> = Vec::new();
     for (key, _value) in file1_hash {
          if file2_hash.contains_key(&key) {
              println!("{}", key);
          }
          else {
              hapax_list.push(key);
          }
     }
    hapax_list
}
