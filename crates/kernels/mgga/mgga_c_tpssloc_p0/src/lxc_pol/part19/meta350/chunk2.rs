//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1274/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1274<F: Float>(t324: F, t41749: F, t41762: F, t10603: F, t2932: F, t10717: F, t10720: F, t10724: F, t10734: F, t10740: F, t10747: F, t10753: F, t10756: F, t10765: F, t10771: F, t10825: F, t10828: F, t14259: F, t2880: F, t2889: F, t2905: F, t2924: F, t2930: F, t2933: F, t41620: F, t41622: F, t41625: F, t41627: F, t41635: F, t41639: F, t41722: F, t950: F) -> (F, F) {
    let t41764 = (t41749 + t41762) * t324;
    let t41769 = t10603 * t2932;
    let t41790 = -F::cast_from(0.19751673498613801407e-1_f64) * t41764 - t41620 - t41622 - t41625 - t41627 - t41635 - t41639 + t41722 - F::cast_from(0.46785788981077169656e1_f64) * t2905 * t10753 * t950 + F::cast_from(0.69263436422725855036e2_f64) * t2930 * t41769 * t950 + F::cast_from(0.61524113149298439947e4_f64) * t10756 * t14259 * t2924 - F::cast_from(0.14035736694323150897e2_f64) * t10747 * t10720 + F::cast_from(0.20779030926817756511e3_f64) * t10825 * t10724 - F::cast_from(0.62337092780453269531e3_f64) * t10828 * t2933 * t2924 - F::cast_from(24.0_f64) * t10740 * t10734 + F::cast_from(0.3859675079686208416e3_f64) * t10765 * t10717 - F::cast_from(0.11579025239058625248e4_f64) * t10771 * t2889 * t2880;
    (t41764, t41790)
}
