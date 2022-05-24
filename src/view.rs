type View = Vec<Vec<char>>;
use super::datas::Datas;

pub(crate) fn create_view(datas: &Datas, message: &String) -> View {
    let width = datas.get_width();
    let depth = datas.get_depth();
    let mut ans: View = vec![vec!['　'; std::cmp::max(width + 2, message.len())]; depth + 2];
    datas.get_frame().iter().for_each(|x| ans[x.0][x.1] = '🔲');
    datas.get_float().iter().for_each(|x| ans[x.0][x.1] = '🟥');
    datas.get_fixed().iter().for_each(|x| ans[x.0][x.1] = '🟦');
    message
        .chars()
        .enumerate()
        .for_each(|(i, c)| ans[depth + 1][i] = c);
    ans
}
