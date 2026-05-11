from __future__ import annotations

from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function

from polars_stdbr._utils import LIB

if TYPE_CHECKING:
    from polars.type_aliases import IntoExprColumn


def cpf_is_valid(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cpf_is_valid",
        args=[expr],
        is_elementwise=True,
    )

def cpf_format(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cpf_format",
        args=[expr],
        is_elementwise=True,
    )

def cpf_remove_symbols(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cpf_remove_symbols",
        args=[expr],
        is_elementwise=True,
    )

def cnpj_is_valid(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cnpj_is_valid",
        args=[expr],
        is_elementwise=True,
    )

def cnpj_format(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cnpj_format",
        args=[expr],
        is_elementwise=True,
    )

def cnpj_remove_symbols(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cnpj_remove_symbols",
        args=[expr],
        is_elementwise=True,
    )

def cep_is_valid(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cep_is_valid",
        args=[expr],
        is_elementwise=True,
    )

def cep_format(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cep_format",
        args=[expr],
        is_elementwise=True,
    )

def cep_remove_symbols(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name="cep_remove_symbols",
        args=[expr],
        is_elementwise=True,
    )

@pl.api.register_expr_namespace("stdbr")
class StdbrExpr:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def cpf_is_valid(self) -> pl.Expr:
        return cpf_is_valid(self._expr)

    def cpf_format(self) -> pl.Expr:
        return cpf_format(self._expr)

    def cpf_remove_symbols(self) -> pl.Expr:
        return cpf_remove_symbols(self._expr)

    def cnpj_is_valid(self) -> pl.Expr:
        return cnpj_is_valid(self._expr)

    def cnpj_format(self) -> pl.Expr:
        return cnpj_format(self._expr)

    def cnpj_remove_symbols(self) -> pl.Expr:
        return cnpj_remove_symbols(self._expr)

    def cep_is_valid(self) -> pl.Expr:
        return cep_is_valid(self._expr)

    def cep_format(self) -> pl.Expr:
        return cep_format(self._expr)

    def cep_remove_symbols(self) -> pl.Expr:
        return cep_remove_symbols(self._expr)
