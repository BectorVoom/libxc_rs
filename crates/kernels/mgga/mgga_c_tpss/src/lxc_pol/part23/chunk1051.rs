//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1051/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1051<F: Float>(t11857: F, t11860: F, t11862: F, t11865: F, t11867: F, t11871: F, t11873: F, t11876: F, t11880: F, t11885: F, t11890: F, t1992: F, t4046: F, t2838: F, t128: F) -> (F, F, F) {
    let t11892 = -0.76790625e-1 * t11857 - 0.1898925e1 * t11860 - 0.9494625e0 * t11862 + 0.3071625e0 * t11865 + 0.15358125e0 * t11867 + 0.49293999999999999999e0 * t11871 + 0.13287407407407407408e0 * t11873 - t11876 + 0.33218518518518518518e0 * t11880 - 0.11958666666666666667e1 * t11885 - 0.39862222222222222222e0 * t11890;
    let t11894 = t4046 * t1992;
    let t11895 = t2838 * t11894;
    let t11896 = t128 * t11895;
    (t11892, t11894, t11896)
}
