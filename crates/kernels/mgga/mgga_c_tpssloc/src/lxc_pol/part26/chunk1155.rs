//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1155/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1155<F: Float>(t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F, t6605: F, t9972: F, t9976: F, t23133: F, t2703: F, t23083: F, t23089: F, t23146: F, t9649: F) -> (F, F, F, F, F, F) {
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81850 = 0.10173934535723378495e0 * t81849;
    let t81852 = t80845 * t206 * t1887;
    let t81853 = 455.0 / 1296.0 * t81852;
    let t81855 = t6605 * t9972 * t9976;
    let t81857 = t23133 * t2703;
    let t81859 = t23083 * t23089;
    let t81861 = t23146 * t9649;
    (t81850, t81853, t81855, t81857, t81859, t81861)
}
