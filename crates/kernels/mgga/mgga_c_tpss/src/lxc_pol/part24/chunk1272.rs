//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1272/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1272<F: Float>(t19434: F, t6103: F, t19620: F, t51631: F, t7029: F, t13798: F, t7310: F, t21028: F, t5706: F, t18547: F, t51642: F, t1659: F, t4397: F, t1760: F, t18289: F, t21027: F) -> (F, F, F, F, F, F, F) {
    let t68936 = 4.0 * t6103 * t19434;
    let t68941 = 6.0 * t19620 * t7029 * t51631;
    let t68944 = 12.0 * t19620 * t7310 * t13798;
    let t68946 = 3.0 * t5706 * t21028;
    let t68949 = 6.0 * t18547 * t7029 * t51642;
    let t68950 = t4397 * t1659;
    let t68953 = 6.0 * t18547 * t7029 * t68950;
    let t68956 = 3.0 * t1760 * t18289 * t21027;
    (t68936, t68941, t68944, t68946, t68949, t68953, t68956)
}
