type View = Vec<Vec<char>>;
use super::datas::Datas;

pub fn create_view(datas: &Datas) -> View {
    let width = datas.get_width();
    let depth = datas.get_depth();
    let mut ans: View = vec![vec!['　'; width + 2]; depth + 1];
    datas.get_frame().iter().for_each(|x| ans[x.0][x.1] = '🔲');
    datas.get_float().iter().for_each(|x| ans[x.0][x.1] = '🟥');
    datas.get_fixed().iter().for_each(|x| ans[x.0][x.1] = '🟦');
    ans
}
