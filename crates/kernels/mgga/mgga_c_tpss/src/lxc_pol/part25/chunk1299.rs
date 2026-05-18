//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1299/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1299<F: Float>(t1275: F, t6458: F, t1673: F, t5941: F, t20697: F, t546: F, t1856: F, t4543: F, t1848: F, t4562: F, t20648: F, t550: F) -> (F, F, F, F, F, F) {
    let t67851 = F::new(2.0) * t1275 * t6458;
    let t67853 = F::new(2.0) * t5941 * t1673;
    let t67858 = F::new(2.0) * t546 * t20697;
    let t67860 = F::new(2.0) * t4543 * t1856;
    let t67868 = F::new(2.0) * t1848 * t4562;
    let t67874 = F::new(2.0) * t20648 * t550;
    (t67851, t67853, t67858, t67860, t67868, t67874)
}
