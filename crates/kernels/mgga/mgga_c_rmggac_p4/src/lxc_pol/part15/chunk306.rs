//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 306/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk306<F: Float>(t1811: F, t183: F, t155: F, t1436: F, t1027: F, t1044: F, t1133: F, t1813: F, t1814: F, t1815: F, t1816: F, t1817: F, t975: F) -> (F, F, F, F) {
    let t1842 = t1811 * t183;
    let t1843 = t155 * t1842;
    let t1844 = F::cast_from(0.36622894612013090108e-3_f64) * t1436;
    let t1845 = t1843 + t1813 + t1815 - t1814 - t1816 - t1817 - t1844 - t1044 - t975 + t1133 - t1027;
    (t1842, t1843, t1844, t1845)
}
