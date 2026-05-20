//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1719/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1719<F: Float>(t40: F, t12943: F, t1409: F, t2517: F, t707: F, t3966: F, t75: F, t12606: F, t1430: F, t2244: F, t2250: F, t4104: F, t607: F, t767: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t12944 = F::cast_from(0.11696447245269292414e1_f64) * t12943;
    let t12945 = t2517 * t1409;
    let t12946 = t707 * t12945;
    let t12947 = F::new(4.0) * t12946;
    let t12950 = t75 * t3966;
    let t12958 = piecewise3::<F>(t146, F::new(0.0), F::new(8.0) / F::new(27.0) * t1430 * t2244 - F::new(4.0) / F::new(9.0) * t12950 * t607 - F::new(2.0) / F::new(9.0) * t4104 * t2250 + F::new(2.0) / F::new(3.0) * t767 * t12606);
    (t12944, t12945, t12947, t12950, t12958)
}
