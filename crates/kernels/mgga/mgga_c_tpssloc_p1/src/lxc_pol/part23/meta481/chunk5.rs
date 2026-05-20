//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1444/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1444<F: Float>(t6144: F, t6138: F, t1174: F, t1409: F, t1710: F, t18321: F, t22035: F, t22041: F, t22056: F, t22060: F, t3447: F, t3450: F, t457: F, t460: F, t4889: F, t4919: F, t6131: F, t65112: F, t65126: F, t73113: F, t974: F) -> F {
    let t78562 = t6144 * t6144;
    let t78568 = t6138 * t6138;
    let t78578 = F::cast_from(0.17777777777777777777e-1_f64) * t4889 * t22060 + F::cast_from(0.50699588477366255142e-1_f64) * t73113 * t1710 - F::cast_from(0.16296296296296296296e-1_f64) * t18321 * t6131 - F::cast_from(0.23703703703703703704e-1_f64) * t4889 * t22056 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t4919 * t3450 * t1409 * t6138 + F::cast_from(0.88888888888888888888e-2_f64) * t4889 * t22035 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t974 * t457 * t78562 * t460 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t974 * t457 * t78568 * t460 + F::cast_from(0.88888888888888888888e-2_f64) * t4889 * t22041 + F::cast_from(0.74074074074074074072e-3_f64) * t65112 - F::cast_from(0.49382716049382716048e-3_f64) * t65126;
    t78578
}
