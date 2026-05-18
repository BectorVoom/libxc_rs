//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 323/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk323<F: Float>(t1949: F, t345: F, t1945: F, t383: F, t1920: F, t353: F, t1055: F, t1052: F, t1923: F, t1946: F, t388: F, t1914: F, t202: F) -> (F, F, F, F, F) {
    let t1950 = t345 * t1949;
    let t1953 = t383 * t1945;
    let t1955 = F::new(0.82246703342411321825e-2) * t1920 * t1950 + t353 * t1953;
    let t1956 = t1055 * t1955;
    let t1958 = F::new(0.82246703342411321825e-2) * t1920 * t1923 + t1946 * t388 - t1052 * t1956;
    let t1962 = t202 * t1914;
    (t1953, t1955, t1956, t1958, t1962)
}
