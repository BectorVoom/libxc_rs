//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1286/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1286<F: Float>(t41855: F, t41878: F, t41931: F, t41975: F, t2853: F, t2860: F, t10770: F, t919: F, t2862: F, t10655: F, t10737: F, t10632: F, t10753: F, t10757: F, t10772: F, t10805: F, t10806: F, t10811: F, t10813: F, t10820: F, t2861: F, t2863: F, t2880: F, t2886: F, t2888: F, t2900: F, t2907: F, t2924: F, t2925: F, t2930: F, t2933: F, t41804: F, t41813: F, t41816: F, t41821: F, t41826: F, t41827: F, t931: F, t943: F, t951: F) -> (F, F, F, F) {
    let t41977 = t41855 + t41878 + t41931 + t41975;
    let t41981 = t2853 * t2860;
    let t41984 = t919 * t10770;
    let t41987 = t2862 * t2862;
    let t41992 = F::cast_from(24.0_f64) * t10655 * t10737;
    let t41993 = -F::cast_from(8.0_f64) * t2861 * t10806 * t931 + F::cast_from(0.12865583598954028054e3_f64) * t2886 * t10805 * t2888 * t931 + F::cast_from(0.12414243100625616072e5_f64) * t10811 * t2862 * t10813 * t2880 - t41804 + F::cast_from(36.0_f64) * t2886 * t2863 * t2880 + F::cast_from(0.21053605041484726346e2_f64) * t2930 * t2907 * t2924 + t41813 + F::cast_from(0.35089341735807877242e1_f64) * t10820 * t2925 + F::cast_from(0.10389515463408878255e3_f64) * t41816 * t2933 + F::cast_from(0.23392894490538584828e1_f64) * t2900 * t10753 + F::cast_from(0.4101607543286562663e4_f64) * t41821 * t10757 - F::cast_from(0.12304822629859687989e5_f64) * t41826 * t41827 * t10632 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t41977 * t951 - F::cast_from(12.0_f64) * t41981 * t2863 - F::cast_from(0.77193501593724168322e3_f64) * t41984 * t10772 + F::cast_from(0.11579025239058625248e4_f64) * t10811 * t41987 * t2888 - t41992;
    (t41977, t41987, t41992, t41993)
}
