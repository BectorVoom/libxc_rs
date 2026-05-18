//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1083/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1083<F: Float>(t11857: F, t11860: F, t11862: F, t11865: F, t11867: F, t11871: F, t11873: F, t11876: F, t11880: F, t11885: F, t11890: F, t1992: F, t4046: F) -> (F, F) {
    let t11892 = -F::new(0.76790625e-1) * t11857 - F::new(0.1898925e1) * t11860 - F::new(0.9494625e0) * t11862 + F::new(0.3071625e0) * t11865 + F::new(0.15358125e0) * t11867 + F::new(0.49293999999999999999e0) * t11871 + F::new(0.13287407407407407408e0) * t11873 - t11876 + F::new(0.33218518518518518518e0) * t11880 - F::new(0.11958666666666666667e1) * t11885 - F::new(0.39862222222222222222e0) * t11890;
    let t11894 = t4046 * t1992;
    (t11892, t11894)
}
