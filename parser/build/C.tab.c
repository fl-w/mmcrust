/* A Bison parser, made by GNU Bison 3.7.2.  */

/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Identify Bison output.  */
#define YYBISON 1

/* Bison version.  */
#define YYBISON_VERSION "3.7.2"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 0

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1




/* First part of user prologue.  */
#line 1 "src/C.y"

#include "nodes.h"
#define YYSTYPE NODE*
#define YYDEBUG 1
extern TOKEN *int_token, *void_token, *function_token, *lasttok;
NODE *ans;

#line 79 "build/C.tab.c"

# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif

#include "C.tab.h"
/* Symbol kind.  */
enum yysymbol_kind_t
{
  YYSYMBOL_YYEMPTY = -2,
  YYSYMBOL_YYEOF = 0,                      /* "end of file"  */
  YYSYMBOL_YYerror = 1,                    /* error  */
  YYSYMBOL_YYUNDEF = 2,                    /* "invalid token"  */
  YYSYMBOL_IDENTIFIER = 3,                 /* IDENTIFIER  */
  YYSYMBOL_CONSTANT = 4,                   /* CONSTANT  */
  YYSYMBOL_STRING_LITERAL = 5,             /* STRING_LITERAL  */
  YYSYMBOL_LE_OP = 6,                      /* LE_OP  */
  YYSYMBOL_GE_OP = 7,                      /* GE_OP  */
  YYSYMBOL_EQ_OP = 8,                      /* EQ_OP  */
  YYSYMBOL_NE_OP = 9,                      /* NE_OP  */
  YYSYMBOL_EXTERN = 10,                    /* EXTERN  */
  YYSYMBOL_AUTO = 11,                      /* AUTO  */
  YYSYMBOL_INT = 12,                       /* INT  */
  YYSYMBOL_VOID = 13,                      /* VOID  */
  YYSYMBOL_FUNCTION = 14,                  /* FUNCTION  */
  YYSYMBOL_APPLY = 15,                     /* APPLY  */
  YYSYMBOL_LEAF = 16,                      /* LEAF  */
  YYSYMBOL_IF = 17,                        /* IF  */
  YYSYMBOL_ELSE = 18,                      /* ELSE  */
  YYSYMBOL_WHILE = 19,                     /* WHILE  */
  YYSYMBOL_CONTINUE = 20,                  /* CONTINUE  */
  YYSYMBOL_BREAK = 21,                     /* BREAK  */
  YYSYMBOL_RETURN = 22,                    /* RETURN  */
  YYSYMBOL_23_ = 23,                       /* '('  */
  YYSYMBOL_24_ = 24,                       /* ')'  */
  YYSYMBOL_25_ = 25,                       /* ','  */
  YYSYMBOL_26_ = 26,                       /* '&'  */
  YYSYMBOL_27_ = 27,                       /* '*'  */
  YYSYMBOL_28_ = 28,                       /* '+'  */
  YYSYMBOL_29_ = 29,                       /* '-'  */
  YYSYMBOL_30_ = 30,                       /* '!'  */
  YYSYMBOL_31_ = 31,                       /* '/'  */
  YYSYMBOL_32_ = 32,                       /* '%'  */
  YYSYMBOL_33_ = 33,                       /* '<'  */
  YYSYMBOL_34_ = 34,                       /* '>'  */
  YYSYMBOL_35_ = 35,                       /* '='  */
  YYSYMBOL_36_ = 36,                       /* ';'  */
  YYSYMBOL_37_ = 37,                       /* '{'  */
  YYSYMBOL_38_ = 38,                       /* '}'  */
  YYSYMBOL_YYACCEPT = 39,                  /* $accept  */
  YYSYMBOL_goal = 40,                      /* goal  */
  YYSYMBOL_primary_expression = 41,        /* primary_expression  */
  YYSYMBOL_postfix_expression = 42,        /* postfix_expression  */
  YYSYMBOL_argument_expression_list = 43,  /* argument_expression_list  */
  YYSYMBOL_unary_expression = 44,          /* unary_expression  */
  YYSYMBOL_unary_operator = 45,            /* unary_operator  */
  YYSYMBOL_multiplicative_expression = 46, /* multiplicative_expression  */
  YYSYMBOL_additive_expression = 47,       /* additive_expression  */
  YYSYMBOL_relational_expression = 48,     /* relational_expression  */
  YYSYMBOL_equality_expression = 49,       /* equality_expression  */
  YYSYMBOL_assignment_expression = 50,     /* assignment_expression  */
  YYSYMBOL_expression = 51,                /* expression  */
  YYSYMBOL_declaration = 52,               /* declaration  */
  YYSYMBOL_declaration_specifiers = 53,    /* declaration_specifiers  */
  YYSYMBOL_init_declarator_list = 54,      /* init_declarator_list  */
  YYSYMBOL_init_declarator = 55,           /* init_declarator  */
  YYSYMBOL_storage_class_specifier = 56,   /* storage_class_specifier  */
  YYSYMBOL_type_specifier = 57,            /* type_specifier  */
  YYSYMBOL_declarator = 58,                /* declarator  */
  YYSYMBOL_direct_declarator = 59,         /* direct_declarator  */
  YYSYMBOL_pointer = 60,                   /* pointer  */
  YYSYMBOL_parameter_list = 61,            /* parameter_list  */
  YYSYMBOL_parameter_declaration = 62,     /* parameter_declaration  */
  YYSYMBOL_identifier_list = 63,           /* identifier_list  */
  YYSYMBOL_abstract_declarator = 64,       /* abstract_declarator  */
  YYSYMBOL_direct_abstract_declarator = 65, /* direct_abstract_declarator  */
  YYSYMBOL_statement = 66,                 /* statement  */
  YYSYMBOL_compound_statement = 67,        /* compound_statement  */
  YYSYMBOL_declaration_list = 68,          /* declaration_list  */
  YYSYMBOL_statement_list = 69,            /* statement_list  */
  YYSYMBOL_expression_statement = 70,      /* expression_statement  */
  YYSYMBOL_selection_statement = 71,       /* selection_statement  */
  YYSYMBOL_iteration_statement = 72,       /* iteration_statement  */
  YYSYMBOL_jump_statement = 73,            /* jump_statement  */
  YYSYMBOL_translation_unit = 74,          /* translation_unit  */
  YYSYMBOL_external_declaration = 75,      /* external_declaration  */
  YYSYMBOL_function_definition = 76        /* function_definition  */
};
typedef enum yysymbol_kind_t yysymbol_kind_t;




#ifdef short
# undef short
#endif

/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */

#ifndef __PTRDIFF_MAX__
# include <limits.h> /* INFRINGES ON USER NAME SPACE */
# if defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stdint.h> /* INFRINGES ON USER NAME SPACE */
#  define YY_STDINT_H
# endif
#endif

/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */

#ifdef __INT_LEAST8_MAX__
typedef __INT_LEAST8_TYPE__ yytype_int8;
#elif defined YY_STDINT_H
typedef int_least8_t yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef __INT_LEAST16_MAX__
typedef __INT_LEAST16_TYPE__ yytype_int16;
#elif defined YY_STDINT_H
typedef int_least16_t yytype_int16;
#else
typedef short yytype_int16;
#endif

#if defined __UINT_LEAST8_MAX__ && __UINT_LEAST8_MAX__ <= __INT_MAX__
typedef __UINT_LEAST8_TYPE__ yytype_uint8;
#elif (!defined __UINT_LEAST8_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST8_MAX <= INT_MAX)
typedef uint_least8_t yytype_uint8;
#elif !defined __UINT_LEAST8_MAX__ && UCHAR_MAX <= INT_MAX
typedef unsigned char yytype_uint8;
#else
typedef short yytype_uint8;
#endif

#if defined __UINT_LEAST16_MAX__ && __UINT_LEAST16_MAX__ <= __INT_MAX__
typedef __UINT_LEAST16_TYPE__ yytype_uint16;
#elif (!defined __UINT_LEAST16_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST16_MAX <= INT_MAX)
typedef uint_least16_t yytype_uint16;
#elif !defined __UINT_LEAST16_MAX__ && USHRT_MAX <= INT_MAX
typedef unsigned short yytype_uint16;
#else
typedef int yytype_uint16;
#endif

#ifndef YYPTRDIFF_T
# if defined __PTRDIFF_TYPE__ && defined __PTRDIFF_MAX__
#  define YYPTRDIFF_T __PTRDIFF_TYPE__
#  define YYPTRDIFF_MAXIMUM __PTRDIFF_MAX__
# elif defined PTRDIFF_MAX
#  ifndef ptrdiff_t
#   include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  endif
#  define YYPTRDIFF_T ptrdiff_t
#  define YYPTRDIFF_MAXIMUM PTRDIFF_MAX
# else
#  define YYPTRDIFF_T long
#  define YYPTRDIFF_MAXIMUM LONG_MAX
# endif
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned
# endif
#endif

#define YYSIZE_MAXIMUM                                  \
  YY_CAST (YYPTRDIFF_T,                                 \
           (YYPTRDIFF_MAXIMUM < YY_CAST (YYSIZE_T, -1)  \
            ? YYPTRDIFF_MAXIMUM                         \
            : YY_CAST (YYSIZE_T, -1)))

#define YYSIZEOF(X) YY_CAST (YYPTRDIFF_T, sizeof (X))


/* Stored state numbers (used for stacks). */
typedef yytype_uint8 yy_state_t;

/* State numbers in computations.  */
typedef int yy_state_fast_t;

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif


#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(E) ((void) (E))
#else
# define YYUSE(E) /* empty */
#endif

#if defined __GNUC__ && ! defined __ICC && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                            \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#define YY_ASSERT(E) ((void) (0 && (E)))

#if !defined yyoverflow

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* !defined yyoverflow */

#if (! defined yyoverflow \
     && (! defined __cplusplus \
         || (defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yy_state_t yyss_alloc;
  YYSTYPE yyvs_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (YYSIZEOF (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (YYSIZEOF (yy_state_t) + YYSIZEOF (YYSTYPE)) \
      + YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYPTRDIFF_T yynewbytes;                                         \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * YYSIZEOF (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / YYSIZEOF (*yyptr);                        \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, YY_CAST (YYSIZE_T, (Count)) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYPTRDIFF_T yyi;                      \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  22
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   380

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  39
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  38
/* YYNRULES -- Number of rules.  */
#define YYNRULES  107
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  171

/* YYMAXUTOK -- Last valid token kind.  */
#define YYMAXUTOK   277


/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
#define YYTRANSLATE(YYX)                                \
  (0 <= (YYX) && (YYX) <= YYMAXUTOK                     \
   ? YY_CAST (yysymbol_kind_t, yytranslate[YYX])        \
   : YYSYMBOL_YYUNDEF)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static const yytype_int8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    30,     2,     2,     2,    32,    26,     2,
      23,    24,    27,    28,    25,    29,     2,    31,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,    36,
      33,    35,    34,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    37,     2,    38,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22
};

#if YYDEBUG
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_int16 yyrline[] =
{
       0,    21,    21,    25,    26,    27,    28,    32,    33,    34,
      39,    40,    45,    46,    50,    51,    52,    53,    54,    58,
      59,    61,    63,    68,    69,    71,    76,    77,    79,    81,
      83,    88,    89,    91,    96,    97,   102,   103,   107,   108,
     109,   114,   115,   117,   118,   122,   123,   127,   128,   132,
     133,   137,   138,   139,   143,   144,   148,   149,   150,   151,
     152,   156,   157,   161,   162,   166,   167,   168,   172,   173,
     179,   180,   181,   185,   186,   187,   188,   189,   193,   194,
     195,   196,   197,   201,   202,   203,   204,   208,   209,   213,
     214,   218,   219,   223,   224,   230,   234,   235,   236,   237,
     241,   242,   246,   247,   251,   253,   255,   257
};
#endif

/** Accessing symbol of state STATE.  */
#define YY_ACCESSING_SYMBOL(State) YY_CAST (yysymbol_kind_t, yystos[State])

#if YYDEBUG || 0
/* The user-facing name of the symbol whose (internal) number is
   YYSYMBOL.  No bounds checking.  */
static const char *yysymbol_name (yysymbol_kind_t yysymbol) YY_ATTRIBUTE_UNUSED;

/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "\"end of file\"", "error", "\"invalid token\"", "IDENTIFIER",
  "CONSTANT", "STRING_LITERAL", "LE_OP", "GE_OP", "EQ_OP", "NE_OP",
  "EXTERN", "AUTO", "INT", "VOID", "FUNCTION", "APPLY", "LEAF", "IF",
  "ELSE", "WHILE", "CONTINUE", "BREAK", "RETURN", "'('", "')'", "','",
  "'&'", "'*'", "'+'", "'-'", "'!'", "'/'", "'%'", "'<'", "'>'", "'='",
  "';'", "'{'", "'}'", "$accept", "goal", "primary_expression",
  "postfix_expression", "argument_expression_list", "unary_expression",
  "unary_operator", "multiplicative_expression", "additive_expression",
  "relational_expression", "equality_expression", "assignment_expression",
  "expression", "declaration", "declaration_specifiers",
  "init_declarator_list", "init_declarator", "storage_class_specifier",
  "type_specifier", "declarator", "direct_declarator", "pointer",
  "parameter_list", "parameter_declaration", "identifier_list",
  "abstract_declarator", "direct_abstract_declarator", "statement",
  "compound_statement", "declaration_list", "statement_list",
  "expression_statement", "selection_statement", "iteration_statement",
  "jump_statement", "translation_unit", "external_declaration",
  "function_definition", YY_NULLPTR
};

static const char *
yysymbol_name (yysymbol_kind_t yysymbol)
{
  return yytname[yysymbol];
}
#endif

#ifdef YYPRINT
/* YYTOKNUM[NUM] -- (External) token number corresponding to the
   (internal) symbol number NUM (which must be that of a token).  */
static const yytype_int16 yytoknum[] =
{
       0,   256,   257,   258,   259,   260,   261,   262,   263,   264,
     265,   266,   267,   268,   269,   270,   271,   272,   273,   274,
     275,   276,   277,    40,    41,    44,    38,    42,    43,    45,
      33,    47,    37,    60,    62,    61,    59,   123,   125
};
#endif

#define YYPACT_NINF (-112)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-57)

#define yytable_value_is_error(Yyn) \
  0

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
     351,  -112,  -112,  -112,  -112,  -112,  -112,    22,    -8,    24,
    -112,    10,   210,   210,    17,    19,    13,   351,  -112,  -112,
       8,  -112,  -112,  -112,   -15,  -112,    71,  -112,  -112,   124,
    -112,  -112,    17,  -112,   356,    19,  -112,  -112,    22,  -112,
     301,  -112,    17,   258,  -112,  -112,    30,    35,    57,    64,
     273,   312,  -112,    -8,  -112,  -112,  -112,  -112,  -112,  -112,
      37,    44,   301,    93,   -11,     5,   109,  -112,    61,  -112,
    -112,   153,   181,  -112,  -112,  -112,  -112,  -112,  -112,  -112,
    -112,    32,   115,  -112,   144,  -112,    67,  -112,   301,  -112,
    -112,  -112,   301,   301,  -112,  -112,  -112,    63,   163,   284,
     301,  -112,   301,   301,   301,   301,   301,   301,   301,   301,
     301,   301,   301,   301,  -112,  -112,   209,  -112,  -112,   333,
    -112,    73,  -112,    82,  -112,   210,  -112,   107,   191,   219,
    -112,  -112,  -112,   224,  -112,  -112,  -112,  -112,  -112,  -112,
      93,    93,   -11,   -11,   -11,   -11,     5,     5,  -112,  -112,
    -112,   226,    88,    82,   182,  -112,  -112,   237,   237,  -112,
     301,  -112,  -112,  -112,   228,    96,  -112,  -112,  -112,   237,
    -112
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_int8 yydefact[] =
{
       0,    56,    49,    50,    52,    51,    53,     0,    61,     0,
     103,     0,    41,    43,     0,    55,     0,     2,   100,    39,
       0,    62,     1,    38,     0,    45,    47,    42,    44,     0,
      87,   107,     0,    39,     0,    54,   101,    57,     0,    40,
       0,   105,     0,     3,     4,     5,     0,     0,     0,     0,
       0,     0,    14,    15,    16,    17,    18,    91,    83,     7,
      12,    19,     0,    23,    26,    31,    34,    36,     0,    89,
      78,     0,     0,    79,    80,    81,    82,    88,   106,    68,
      60,    67,     0,    63,     0,    46,    47,     3,     0,    15,
      48,   104,     0,     0,    96,    97,    98,     0,     0,     0,
       0,    13,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    92,    85,     0,    84,    90,     0,
      65,    70,    66,    71,    58,     0,    59,     0,     0,     0,
      99,     6,     8,     0,    10,    35,    20,    21,    22,    19,
      24,    25,    29,    30,    27,    28,    32,    33,    37,    86,
      74,     0,     0,    72,     0,    64,    69,     0,     0,     9,
       0,    75,    73,    76,     0,    93,    95,    11,    77,     0,
      94
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -112,  -112,  -112,  -112,  -112,   -39,  -112,    43,    23,   122,
    -112,   -38,   -36,     9,   -12,  -112,    83,  -112,  -112,    -4,
     -10,    -3,  -111,    -2,  -112,     7,    34,   -68,    77,    66,
     100,  -112,  -112,  -112,  -112,  -112,   142,    87
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
      -1,     9,    59,    60,   133,    61,    62,    63,    64,    65,
      66,    67,    68,    30,    11,    24,    25,    12,    13,    14,
      15,    16,    82,    83,    84,   122,   123,    69,    70,    32,
      72,    73,    74,    75,    76,    17,    18,    33
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_int16 yytable[] =
{
      27,    28,    90,    20,   118,    21,    35,    26,   151,    10,
      38,   107,   108,     1,    97,    98,     1,   105,   106,     8,
       1,    39,    81,   101,    22,     1,    10,     2,     3,     4,
       5,     6,    37,     7,    86,     1,     7,     8,   109,   110,
       7,    77,    34,   164,     8,     7,    23,    20,   118,     8,
      21,    77,    98,    92,    29,   119,   128,   129,    93,     8,
      99,   134,   135,   136,   137,   138,   139,   139,   139,   139,
     139,   139,   139,   139,     1,   148,     1,   120,   121,   100,
      77,     2,     3,     4,     5,     6,   113,    19,   113,   165,
     166,    31,    42,    94,     7,    71,   119,   114,     8,   130,
      95,   170,    40,    41,    19,   154,    40,    81,    29,    78,
     156,    35,   162,    81,   169,    20,   121,   111,   112,    91,
     102,    85,   167,   155,   103,   104,   152,    43,    44,    45,
     142,   143,   144,   145,     2,     3,     4,     5,     6,   124,
     125,    46,    81,    47,    48,    49,    50,    51,   140,   141,
      52,    53,    54,    55,    56,   153,    43,    44,    45,    36,
      57,    29,    58,     2,     3,     4,     5,     6,   126,   127,
      46,   116,    47,    48,    49,    50,    51,     0,     0,    52,
      53,    54,    55,    56,    87,    44,    45,   131,   113,    57,
      29,   115,     2,     3,     4,     5,     6,     0,    46,     0,
      47,    48,    49,    50,    88,     0,   163,    52,    89,    54,
      55,    56,    87,    44,    45,   157,   113,    57,    29,   117,
       2,     3,     4,     5,     6,     0,    46,     0,    47,    48,
      49,    50,    88,   146,   147,    52,    89,    54,    55,    56,
      87,    44,    45,   158,   113,    57,    29,   149,   159,   160,
     161,   125,   168,   125,    46,     0,    47,    48,    49,    50,
      88,   -56,     0,    52,    89,    54,    55,    56,   -56,   -56,
     -56,   -56,   -56,    57,    29,     0,    87,    44,    45,     0,
       0,     0,     0,     0,     0,     0,     0,    87,    44,    45,
       0,     0,     0,     0,     0,   -56,    88,     0,     0,    52,
      89,    54,    55,    56,    87,    44,    45,    88,   132,    96,
      52,    89,    54,    55,    56,    43,    44,    45,     0,     0,
       0,     0,     0,     0,    88,     0,     0,    52,    89,    54,
      55,    56,     0,     0,     0,    51,     1,     0,    52,    53,
      54,    55,    56,     2,     3,     4,     5,     6,     0,     0,
       0,     0,     0,     0,     1,     0,   119,   150,     0,    79,
       8,     2,     3,     4,     5,     6,     2,     3,     4,     5,
       6,     0,     0,     0,     7,     0,     0,     0,     8,     0,
      80
};

static const yytype_int16 yycheck[] =
{
      12,    13,    40,     7,    72,     8,    16,    11,   119,     0,
      25,     6,     7,     3,    50,    51,     3,    28,    29,    27,
       3,    36,    34,    62,     0,     3,    17,    10,    11,    12,
      13,    14,    24,    23,    38,     3,    23,    27,    33,    34,
      23,    32,    23,   154,    27,    23,    36,    51,   116,    27,
      53,    42,    88,    23,    37,    23,    92,    93,    23,    27,
      23,    99,   100,   102,   103,   104,   105,   106,   107,   108,
     109,   110,   111,   112,     3,   113,     3,    81,    81,    35,
      71,    10,    11,    12,    13,    14,    25,     0,    25,   157,
     158,    14,    26,    36,    23,    29,    23,    36,    27,    36,
      36,   169,    35,    26,    17,    23,    35,   119,    37,    32,
       3,   121,    24,   125,    18,   119,   119,     8,     9,    42,
      27,    38,   160,   125,    31,    32,   119,     3,     4,     5,
     107,   108,   109,   110,    10,    11,    12,    13,    14,    24,
      25,    17,   154,    19,    20,    21,    22,    23,   105,   106,
      26,    27,    28,    29,    30,   121,     3,     4,     5,    17,
      36,    37,    38,    10,    11,    12,    13,    14,    24,    25,
      17,    71,    19,    20,    21,    22,    23,    -1,    -1,    26,
      27,    28,    29,    30,     3,     4,     5,    24,    25,    36,
      37,    38,    10,    11,    12,    13,    14,    -1,    17,    -1,
      19,    20,    21,    22,    23,    -1,    24,    26,    27,    28,
      29,    30,     3,     4,     5,    24,    25,    36,    37,    38,
      10,    11,    12,    13,    14,    -1,    17,    -1,    19,    20,
      21,    22,    23,   111,   112,    26,    27,    28,    29,    30,
       3,     4,     5,    24,    25,    36,    37,    38,    24,    25,
      24,    25,    24,    25,    17,    -1,    19,    20,    21,    22,
      23,     3,    -1,    26,    27,    28,    29,    30,    10,    11,
      12,    13,    14,    36,    37,    -1,     3,     4,     5,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,     4,     5,
      -1,    -1,    -1,    -1,    -1,    37,    23,    -1,    -1,    26,
      27,    28,    29,    30,     3,     4,     5,    23,    24,    36,
      26,    27,    28,    29,    30,     3,     4,     5,    -1,    -1,
      -1,    -1,    -1,    -1,    23,    -1,    -1,    26,    27,    28,
      29,    30,    -1,    -1,    -1,    23,     3,    -1,    26,    27,
      28,    29,    30,    10,    11,    12,    13,    14,    -1,    -1,
      -1,    -1,    -1,    -1,     3,    -1,    23,    24,    -1,     3,
      27,    10,    11,    12,    13,    14,    10,    11,    12,    13,
      14,    -1,    -1,    -1,    23,    -1,    -1,    -1,    27,    -1,
      24
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_int8 yystos[] =
{
       0,     3,    10,    11,    12,    13,    14,    23,    27,    40,
      52,    53,    56,    57,    58,    59,    60,    74,    75,    76,
      58,    60,     0,    36,    54,    55,    58,    53,    53,    37,
      52,    67,    68,    76,    23,    59,    75,    24,    25,    36,
      35,    67,    68,     3,     4,     5,    17,    19,    20,    21,
      22,    23,    26,    27,    28,    29,    30,    36,    38,    41,
      42,    44,    45,    46,    47,    48,    49,    50,    51,    66,
      67,    68,    69,    70,    71,    72,    73,    52,    67,     3,
      24,    53,    61,    62,    63,    55,    58,     3,    23,    27,
      50,    67,    23,    23,    36,    36,    36,    51,    51,    23,
      35,    44,    27,    31,    32,    28,    29,     6,     7,    33,
      34,     8,     9,    25,    36,    38,    69,    38,    66,    23,
      58,    60,    64,    65,    24,    25,    24,    25,    51,    51,
      36,    24,    24,    43,    50,    50,    44,    44,    44,    44,
      46,    46,    47,    47,    47,    47,    48,    48,    50,    38,
      24,    61,    64,    65,    23,    62,     3,    24,    24,    24,
      25,    24,    24,    24,    61,    66,    66,    50,    24,    18,
      66
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_int8 yyr1[] =
{
       0,    39,    40,    41,    41,    41,    41,    42,    42,    42,
      43,    43,    44,    44,    45,    45,    45,    45,    45,    46,
      46,    46,    46,    47,    47,    47,    48,    48,    48,    48,
      48,    49,    49,    49,    50,    50,    51,    51,    52,    52,
      52,    53,    53,    53,    53,    54,    54,    55,    55,    56,
      56,    57,    57,    57,    58,    58,    59,    59,    59,    59,
      59,    60,    60,    61,    61,    62,    62,    62,    63,    63,
      64,    64,    64,    65,    65,    65,    65,    65,    66,    66,
      66,    66,    66,    67,    67,    67,    67,    68,    68,    69,
      69,    70,    70,    71,    71,    72,    73,    73,    73,    73,
      74,    74,    75,    75,    76,    76,    76,    76
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     1,     1,     1,     1,     3,     1,     3,     4,
       1,     3,     1,     2,     1,     1,     1,     1,     1,     1,
       3,     3,     3,     1,     3,     3,     1,     3,     3,     3,
       3,     1,     3,     3,     1,     3,     1,     3,     2,     1,
       3,     1,     2,     1,     2,     1,     3,     1,     3,     1,
       1,     1,     1,     1,     2,     1,     1,     3,     4,     4,
       3,     1,     2,     1,     3,     2,     2,     1,     1,     3,
       1,     1,     2,     3,     2,     3,     3,     4,     1,     1,
       1,     1,     1,     2,     3,     3,     4,     1,     2,     1,
       2,     1,     2,     5,     7,     5,     2,     2,     2,     3,
       1,     2,     1,     1,     4,     3,     3,     2
};


enum { YYENOMEM = -2 };

#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = YYEMPTY)

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                    \
  do                                                              \
    if (yychar == YYEMPTY)                                        \
      {                                                           \
        yychar = (Token);                                         \
        yylval = (Value);                                         \
        YYPOPSTACK (yylen);                                       \
        yystate = *yyssp;                                         \
        goto yybackup;                                            \
      }                                                           \
    else                                                          \
      {                                                           \
        yyerror (YY_("syntax error: cannot back up")); \
        YYERROR;                                                  \
      }                                                           \
  while (0)

/* Backward compatibility with an undocumented macro.
   Use YYerror or YYUNDEF. */
#define YYERRCODE YYUNDEF


/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)

/* This macro is provided for backward compatibility. */
# ifndef YY_LOCATION_PRINT
#  define YY_LOCATION_PRINT(File, Loc) ((void) 0)
# endif


# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Kind, Value); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*-----------------------------------.
| Print this symbol's value on YYO.  |
`-----------------------------------*/

static void
yy_symbol_value_print (FILE *yyo,
                       yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep)
{
  FILE *yyoutput = yyo;
  YYUSE (yyoutput);
  if (!yyvaluep)
    return;
# ifdef YYPRINT
  if (yykind < YYNTOKENS)
    YYPRINT (yyo, yytoknum[yykind], *yyvaluep);
# endif
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/*---------------------------.
| Print this symbol on YYO.  |
`---------------------------*/

static void
yy_symbol_print (FILE *yyo,
                 yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep)
{
  YYFPRINTF (yyo, "%s %s (",
             yykind < YYNTOKENS ? "token" : "nterm", yysymbol_name (yykind));

  yy_symbol_value_print (yyo, yykind, yyvaluep);
  YYFPRINTF (yyo, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yy_state_t *yybottom, yy_state_t *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yy_state_t *yyssp, YYSTYPE *yyvsp,
                 int yyrule)
{
  int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %d):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       YY_ACCESSING_SYMBOL (+yyssp[yyi + 1 - yynrhs]),
                       &yyvsp[(yyi + 1) - (yynrhs)]);
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, Rule); \
} while (0)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args) ((void) 0)
# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif






/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg,
            yysymbol_kind_t yykind, YYSTYPE *yyvaluep)
{
  YYUSE (yyvaluep);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yykind, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/* Lookahead token kind.  */
int yychar;

/* The semantic value of the lookahead symbol.  */
YYSTYPE yylval;
/* Number of syntax errors so far.  */
int yynerrs;




/*----------.
| yyparse.  |
`----------*/

int
yyparse (void)
{
    yy_state_fast_t yystate = 0;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus = 0;

    /* Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* Their size.  */
    YYPTRDIFF_T yystacksize = YYINITDEPTH;

    /* The state stack: array, bottom, top.  */
    yy_state_t yyssa[YYINITDEPTH];
    yy_state_t *yyss = yyssa;
    yy_state_t *yyssp = yyss;

    /* The semantic value stack: array, bottom, top.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs = yyvsa;
    YYSTYPE *yyvsp = yyvs;

  int yyn;
  /* The return value of yyparse.  */
  int yyresult;
  /* Lookahead symbol kind.  */
  yysymbol_kind_t yytoken = YYSYMBOL_YYEMPTY;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;



#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yychar = YYEMPTY; /* Cause a token to be read.  */
  goto yysetstate;


/*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;


/*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
yysetstate:
  YYDPRINTF ((stderr, "Entering state %d\n", yystate));
  YY_ASSERT (0 <= yystate && yystate < YYNSTATES);
  YY_IGNORE_USELESS_CAST_BEGIN
  *yyssp = YY_CAST (yy_state_t, yystate);
  YY_IGNORE_USELESS_CAST_END
  YY_STACK_PRINT (yyss, yyssp);

  if (yyss + yystacksize - 1 <= yyssp)
#if !defined yyoverflow && !defined YYSTACK_RELOCATE
    goto yyexhaustedlab;
#else
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYPTRDIFF_T yysize = yyssp - yyss + 1;

# if defined yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        yy_state_t *yyss1 = yyss;
        YYSTYPE *yyvs1 = yyvs;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * YYSIZEOF (*yyssp),
                    &yyvs1, yysize * YYSIZEOF (*yyvsp),
                    &yystacksize);
        yyss = yyss1;
        yyvs = yyvs1;
      }
# else /* defined YYSTACK_RELOCATE */
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yy_state_t *yyss1 = yyss;
        union yyalloc *yyptr =
          YY_CAST (union yyalloc *,
                   YYSTACK_ALLOC (YY_CAST (YYSIZE_T, YYSTACK_BYTES (yystacksize))));
        if (! yyptr)
          goto yyexhaustedlab;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
#  undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;

      YY_IGNORE_USELESS_CAST_BEGIN
      YYDPRINTF ((stderr, "Stack size increased to %ld\n",
                  YY_CAST (long, yystacksize)));
      YY_IGNORE_USELESS_CAST_END

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }
#endif /* !defined yyoverflow && !defined YYSTACK_RELOCATE */

  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;


/*-----------.
| yybackup.  |
`-----------*/
yybackup:
  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either empty, or end-of-input, or a valid lookahead.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token\n"));
      yychar = yylex ();
    }

  if (yychar <= YYEOF)
    {
      yychar = YYEOF;
      yytoken = YYSYMBOL_YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else if (yychar == YYerror)
    {
      /* The scanner already issued an error message, process directly
         to error recovery.  But do not keep the error token as
         lookahead, it is too special and may lead us to an endless
         loop in error recovery. */
      yychar = YYUNDEF;
      yytoken = YYSYMBOL_YYerror;
      goto yyerrlab1;
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);
  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  /* Discard the shifted token.  */
  yychar = YYEMPTY;
  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];


  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
  case 2: /* goal: translation_unit  */
#line 21 "src/C.y"
                            { ans = yyval = yyvsp[0];}
#line 1307 "build/C.tab.c"
    break;

  case 3: /* primary_expression: IDENTIFIER  */
#line 25 "src/C.y"
                                        { yyval = make_leaf(lasttok); }
#line 1313 "build/C.tab.c"
    break;

  case 4: /* primary_expression: CONSTANT  */
#line 26 "src/C.y"
                                        { yyval = make_leaf(lasttok); }
#line 1319 "build/C.tab.c"
    break;

  case 5: /* primary_expression: STRING_LITERAL  */
#line 27 "src/C.y"
                                        { yyval = make_leaf(lasttok); }
#line 1325 "build/C.tab.c"
    break;

  case 6: /* primary_expression: '(' expression ')'  */
#line 28 "src/C.y"
                                        { yyval = yyvsp[-1]; }
#line 1331 "build/C.tab.c"
    break;

  case 7: /* postfix_expression: primary_expression  */
#line 32 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1337 "build/C.tab.c"
    break;

  case 8: /* postfix_expression: postfix_expression '(' ')'  */
#line 33 "src/C.y"
                                        { yyval = make_node(APPLY, yyvsp[-2], NULL); }
#line 1343 "build/C.tab.c"
    break;

  case 9: /* postfix_expression: postfix_expression '(' argument_expression_list ')'  */
#line 34 "src/C.y"
                                                              {
				          yyval = make_node(APPLY, yyvsp[-3], yyvsp[-1]); }
#line 1350 "build/C.tab.c"
    break;

  case 10: /* argument_expression_list: assignment_expression  */
#line 39 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1356 "build/C.tab.c"
    break;

  case 11: /* argument_expression_list: argument_expression_list ',' assignment_expression  */
#line 40 "src/C.y"
                                                             {
          yyval = make_node(',', yyvsp[-2], yyvsp[0]); }
#line 1363 "build/C.tab.c"
    break;

  case 12: /* unary_expression: postfix_expression  */
#line 45 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1369 "build/C.tab.c"
    break;

  case 13: /* unary_expression: unary_operator unary_expression  */
#line 46 "src/C.y"
                                          { yyval = make_node((int)yyvsp[-1], yyvsp[0], NULL); }
#line 1375 "build/C.tab.c"
    break;

  case 14: /* unary_operator: '&'  */
#line 50 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1381 "build/C.tab.c"
    break;

  case 15: /* unary_operator: '*'  */
#line 51 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1387 "build/C.tab.c"
    break;

  case 16: /* unary_operator: '+'  */
#line 52 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1393 "build/C.tab.c"
    break;

  case 17: /* unary_operator: '-'  */
#line 53 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1399 "build/C.tab.c"
    break;

  case 18: /* unary_operator: '!'  */
#line 54 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1405 "build/C.tab.c"
    break;

  case 19: /* multiplicative_expression: unary_expression  */
#line 58 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1411 "build/C.tab.c"
    break;

  case 20: /* multiplicative_expression: multiplicative_expression '*' unary_expression  */
#line 59 "src/C.y"
                                                         {
                                          yyval = make_node('*', yyvsp[-2], yyvsp[0]); }
#line 1418 "build/C.tab.c"
    break;

  case 21: /* multiplicative_expression: multiplicative_expression '/' unary_expression  */
#line 61 "src/C.y"
                                                         {
                                          yyval = make_node('/', yyvsp[-2], yyvsp[0]); }
#line 1425 "build/C.tab.c"
    break;

  case 22: /* multiplicative_expression: multiplicative_expression '%' unary_expression  */
#line 63 "src/C.y"
                                                         {
                                          yyval = make_node('%', yyvsp[-2], yyvsp[0]); }
#line 1432 "build/C.tab.c"
    break;

  case 23: /* additive_expression: multiplicative_expression  */
#line 68 "src/C.y"
                                                { yyval = yyvsp[0]; }
#line 1438 "build/C.tab.c"
    break;

  case 24: /* additive_expression: additive_expression '+' multiplicative_expression  */
#line 69 "src/C.y"
                                                            {
                                          yyval = make_node('+', yyvsp[-2], yyvsp[0]); }
#line 1445 "build/C.tab.c"
    break;

  case 25: /* additive_expression: additive_expression '-' multiplicative_expression  */
#line 71 "src/C.y"
                                                            {
                                          yyval = make_node('-', yyvsp[-2], yyvsp[0]); }
#line 1452 "build/C.tab.c"
    break;

  case 26: /* relational_expression: additive_expression  */
#line 76 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1458 "build/C.tab.c"
    break;

  case 27: /* relational_expression: relational_expression '<' additive_expression  */
#line 77 "src/C.y"
                                                        {
                                          yyval = make_node('<', yyvsp[-2], yyvsp[0]); }
#line 1465 "build/C.tab.c"
    break;

  case 28: /* relational_expression: relational_expression '>' additive_expression  */
#line 79 "src/C.y"
                                                        {
                                          yyval = make_node('>', yyvsp[-2], yyvsp[0]); }
#line 1472 "build/C.tab.c"
    break;

  case 29: /* relational_expression: relational_expression LE_OP additive_expression  */
#line 81 "src/C.y"
                                                          {
                                          yyval = make_node(LE_OP, yyvsp[-2], yyvsp[0]); }
#line 1479 "build/C.tab.c"
    break;

  case 30: /* relational_expression: relational_expression GE_OP additive_expression  */
#line 83 "src/C.y"
                                                          {
                                          yyval = make_node(GE_OP, yyvsp[-2], yyvsp[0]); }
#line 1486 "build/C.tab.c"
    break;

  case 31: /* equality_expression: relational_expression  */
#line 88 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1492 "build/C.tab.c"
    break;

  case 32: /* equality_expression: equality_expression EQ_OP relational_expression  */
#line 89 "src/C.y"
                                                          {
                                          yyval = make_node(EQ_OP, yyvsp[-2], yyvsp[0]); }
#line 1499 "build/C.tab.c"
    break;

  case 33: /* equality_expression: equality_expression NE_OP relational_expression  */
#line 91 "src/C.y"
                                                          {
                                          yyval = make_node(NE_OP, yyvsp[-2], yyvsp[0]); }
#line 1506 "build/C.tab.c"
    break;

  case 34: /* assignment_expression: equality_expression  */
#line 96 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1512 "build/C.tab.c"
    break;

  case 35: /* assignment_expression: unary_expression '=' assignment_expression  */
#line 97 "src/C.y"
                                                     {
                                          yyval = make_node('=', yyvsp[-2], yyvsp[0]); }
#line 1519 "build/C.tab.c"
    break;

  case 36: /* expression: assignment_expression  */
#line 102 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1525 "build/C.tab.c"
    break;

  case 37: /* expression: expression ',' assignment_expression  */
#line 103 "src/C.y"
                                               { yyval = make_node(',', yyvsp[-2], yyvsp[0]); }
#line 1531 "build/C.tab.c"
    break;

  case 38: /* declaration: declaration_specifiers ';'  */
#line 107 "src/C.y"
                                                { yyval = yyvsp[-1]; }
#line 1537 "build/C.tab.c"
    break;

  case 39: /* declaration: function_definition  */
#line 108 "src/C.y"
                                                { yyval = yyvsp[0]; }
#line 1543 "build/C.tab.c"
    break;

  case 40: /* declaration: declaration_specifiers init_declarator_list ';'  */
#line 109 "src/C.y"
                                                          {
                                                  yyval = make_node('~', yyvsp[-2], yyvsp[-1]); }
#line 1550 "build/C.tab.c"
    break;

  case 41: /* declaration_specifiers: storage_class_specifier  */
#line 114 "src/C.y"
                                                { yyval = yyvsp[0]; }
#line 1556 "build/C.tab.c"
    break;

  case 42: /* declaration_specifiers: storage_class_specifier declaration_specifiers  */
#line 115 "src/C.y"
                                                         { 
                                                  yyval = make_node('~', yyvsp[-1], yyvsp[0]); }
#line 1563 "build/C.tab.c"
    break;

  case 43: /* declaration_specifiers: type_specifier  */
#line 117 "src/C.y"
                                                { yyval = yyvsp[0]; }
#line 1569 "build/C.tab.c"
    break;

  case 44: /* declaration_specifiers: type_specifier declaration_specifiers  */
#line 118 "src/C.y"
                                                { yyval = make_node('~', yyvsp[-1], yyvsp[0]); }
#line 1575 "build/C.tab.c"
    break;

  case 45: /* init_declarator_list: init_declarator  */
#line 122 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1581 "build/C.tab.c"
    break;

  case 46: /* init_declarator_list: init_declarator_list ',' init_declarator  */
#line 123 "src/C.y"
                                                   { yyval = make_node(',', yyvsp[-2], yyvsp[0]); }
#line 1587 "build/C.tab.c"
    break;

  case 47: /* init_declarator: declarator  */
#line 127 "src/C.y"
                                { yyval = yyvsp[0]; }
#line 1593 "build/C.tab.c"
    break;

  case 48: /* init_declarator: declarator '=' assignment_expression  */
#line 128 "src/C.y"
                                               { yyval = make_node('=', yyvsp[-2], yyvsp[0]); }
#line 1599 "build/C.tab.c"
    break;

  case 49: /* storage_class_specifier: EXTERN  */
#line 132 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1605 "build/C.tab.c"
    break;

  case 50: /* storage_class_specifier: AUTO  */
#line 133 "src/C.y"
                        { yyval = yyvsp[0]; }
#line 1611 "build/C.tab.c"
    break;

  case 51: /* type_specifier: VOID  */
#line 137 "src/C.y"
                        { yyval = make_leaf(void_token); }
#line 1617 "build/C.tab.c"
    break;

  case 52: /* type_specifier: INT  */
#line 138 "src/C.y"
                        { yyval = make_leaf(int_token); }
#line 1623 "build/C.tab.c"
    break;

  case 53: /* type_specifier: FUNCTION  */
#line 139 "src/C.y"
                        { yyval = make_leaf(function_token); }
#line 1629 "build/C.tab.c"
    break;

  case 54: /* declarator: pointer direct_declarator  */
#line 143 "src/C.y"
                                        { yyval = make_node('~', yyvsp[-1], yyvsp[0]); }
#line 1635 "build/C.tab.c"
    break;

  case 55: /* declarator: direct_declarator  */
#line 144 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1641 "build/C.tab.c"
    break;

  case 56: /* direct_declarator: IDENTIFIER  */
#line 148 "src/C.y"
                                { yyval = make_leaf(lasttok); }
#line 1647 "build/C.tab.c"
    break;

  case 57: /* direct_declarator: '(' declarator ')'  */
#line 149 "src/C.y"
                                { yyval = yyvsp[-1]; }
#line 1653 "build/C.tab.c"
    break;

  case 58: /* direct_declarator: direct_declarator '(' parameter_list ')'  */
#line 150 "src/C.y"
                                                   { yyval = make_node('F', yyvsp[-3], yyvsp[-1]); }
#line 1659 "build/C.tab.c"
    break;

  case 59: /* direct_declarator: direct_declarator '(' identifier_list ')'  */
#line 151 "src/C.y"
                                                   { yyval = make_node('F', yyvsp[-3], yyvsp[-1]); }
#line 1665 "build/C.tab.c"
    break;

  case 60: /* direct_declarator: direct_declarator '(' ')'  */
#line 152 "src/C.y"
                                                   { yyval = make_node('F', yyvsp[-2], NULL); }
#line 1671 "build/C.tab.c"
    break;

  case 61: /* pointer: '*'  */
#line 156 "src/C.y"
                                { yyval = (NODE*)1; }
#line 1677 "build/C.tab.c"
    break;

  case 62: /* pointer: '*' pointer  */
#line 157 "src/C.y"
                                { yyval = (NODE*)((int)yyvsp[0]+1); }
#line 1683 "build/C.tab.c"
    break;

  case 63: /* parameter_list: parameter_declaration  */
#line 161 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1689 "build/C.tab.c"
    break;

  case 64: /* parameter_list: parameter_list ',' parameter_declaration  */
#line 162 "src/C.y"
                                                   { yyval = make_node(',', yyvsp[-2], yyvsp[0]); }
#line 1695 "build/C.tab.c"
    break;

  case 65: /* parameter_declaration: declaration_specifiers declarator  */
#line 166 "src/C.y"
                                            { yyval = make_node('~', yyvsp[-1], yyvsp[0]); }
#line 1701 "build/C.tab.c"
    break;

  case 66: /* parameter_declaration: declaration_specifiers abstract_declarator  */
#line 167 "src/C.y"
                                                     { yyval = make_node('~', yyvsp[-1], yyvsp[0]); }
#line 1707 "build/C.tab.c"
    break;

  case 67: /* parameter_declaration: declaration_specifiers  */
#line 168 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1713 "build/C.tab.c"
    break;

  case 68: /* identifier_list: IDENTIFIER  */
#line 172 "src/C.y"
                                        { yyval = make_leaf(lasttok); }
#line 1719 "build/C.tab.c"
    break;

  case 69: /* identifier_list: identifier_list ',' IDENTIFIER  */
#line 173 "src/C.y"
                                         {
                                          yyval = make_node(',', yyvsp[-2],
                                                              make_leaf(lasttok)); }
#line 1727 "build/C.tab.c"
    break;

  case 70: /* abstract_declarator: pointer  */
#line 179 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1733 "build/C.tab.c"
    break;

  case 71: /* abstract_declarator: direct_abstract_declarator  */
#line 180 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1739 "build/C.tab.c"
    break;

  case 72: /* abstract_declarator: pointer direct_abstract_declarator  */
#line 181 "src/C.y"
                                             { yyval = make_node('G', yyvsp[-1], yyvsp[0]); }
#line 1745 "build/C.tab.c"
    break;

  case 73: /* direct_abstract_declarator: '(' abstract_declarator ')'  */
#line 185 "src/C.y"
                                         { yyval = yyvsp[-1]; }
#line 1751 "build/C.tab.c"
    break;

  case 74: /* direct_abstract_declarator: '(' ')'  */
#line 186 "src/C.y"
                     { yyval = NULL; }
#line 1757 "build/C.tab.c"
    break;

  case 75: /* direct_abstract_declarator: '(' parameter_list ')'  */
#line 187 "src/C.y"
                                    { yyval = yyvsp[-1]; }
#line 1763 "build/C.tab.c"
    break;

  case 76: /* direct_abstract_declarator: direct_abstract_declarator '(' ')'  */
#line 188 "src/C.y"
                                                { yyval = make_node(APPLY, yyvsp[-2], NULL); }
#line 1769 "build/C.tab.c"
    break;

  case 77: /* direct_abstract_declarator: direct_abstract_declarator '(' parameter_list ')'  */
#line 189 "src/C.y"
                                                              { yyval = make_node(APPLY, yyvsp[-3], yyvsp[-1]); }
#line 1775 "build/C.tab.c"
    break;

  case 78: /* statement: compound_statement  */
#line 193 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1781 "build/C.tab.c"
    break;

  case 79: /* statement: expression_statement  */
#line 194 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1787 "build/C.tab.c"
    break;

  case 80: /* statement: selection_statement  */
#line 195 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1793 "build/C.tab.c"
    break;

  case 81: /* statement: iteration_statement  */
#line 196 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1799 "build/C.tab.c"
    break;

  case 82: /* statement: jump_statement  */
#line 197 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1805 "build/C.tab.c"
    break;

  case 83: /* compound_statement: '{' '}'  */
#line 201 "src/C.y"
                                        { yyval = NULL; }
#line 1811 "build/C.tab.c"
    break;

  case 84: /* compound_statement: '{' statement_list '}'  */
#line 202 "src/C.y"
                                        { yyval = yyvsp[-1]; }
#line 1817 "build/C.tab.c"
    break;

  case 85: /* compound_statement: '{' declaration_list '}'  */
#line 203 "src/C.y"
                                        { yyval = yyvsp[-1]; }
#line 1823 "build/C.tab.c"
    break;

  case 86: /* compound_statement: '{' declaration_list statement_list '}'  */
#line 204 "src/C.y"
                                                  { yyval = make_node(';', yyvsp[-2], yyvsp[-1]); }
#line 1829 "build/C.tab.c"
    break;

  case 87: /* declaration_list: declaration  */
#line 208 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1835 "build/C.tab.c"
    break;

  case 88: /* declaration_list: declaration_list declaration  */
#line 209 "src/C.y"
                                       { yyval = make_node(';', yyvsp[-1], yyvsp[0]); }
#line 1841 "build/C.tab.c"
    break;

  case 89: /* statement_list: statement  */
#line 213 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1847 "build/C.tab.c"
    break;

  case 90: /* statement_list: statement_list statement  */
#line 214 "src/C.y"
                                        { yyval = make_node(';', yyvsp[-1], yyvsp[0]); }
#line 1853 "build/C.tab.c"
    break;

  case 91: /* expression_statement: ';'  */
#line 218 "src/C.y"
                                        { yyval = NULL; }
#line 1859 "build/C.tab.c"
    break;

  case 92: /* expression_statement: expression ';'  */
#line 219 "src/C.y"
                                        { yyval = yyvsp[-1]; }
#line 1865 "build/C.tab.c"
    break;

  case 93: /* selection_statement: IF '(' expression ')' statement  */
#line 223 "src/C.y"
                                          { yyval = make_node(IF, yyvsp[-2], yyvsp[0]); }
#line 1871 "build/C.tab.c"
    break;

  case 94: /* selection_statement: IF '(' expression ')' statement ELSE statement  */
#line 225 "src/C.y"
                                          { yyval = make_node(IF, yyvsp[-4],
                                                        make_node(ELSE, yyvsp[-2], yyvsp[0])); }
#line 1878 "build/C.tab.c"
    break;

  case 95: /* iteration_statement: WHILE '(' expression ')' statement  */
#line 230 "src/C.y"
                                             { yyval = make_node(WHILE, yyvsp[-2], yyvsp[0]); }
#line 1884 "build/C.tab.c"
    break;

  case 96: /* jump_statement: CONTINUE ';'  */
#line 234 "src/C.y"
                                        { yyval = make_node(CONTINUE, NULL, NULL); }
#line 1890 "build/C.tab.c"
    break;

  case 97: /* jump_statement: BREAK ';'  */
#line 235 "src/C.y"
                                        { yyval = make_node(BREAK, NULL, NULL); }
#line 1896 "build/C.tab.c"
    break;

  case 98: /* jump_statement: RETURN ';'  */
#line 236 "src/C.y"
                                        { yyval = make_node(RETURN, NULL, NULL); }
#line 1902 "build/C.tab.c"
    break;

  case 99: /* jump_statement: RETURN expression ';'  */
#line 237 "src/C.y"
                                        { yyval = make_node(RETURN, yyvsp[-1], NULL); }
#line 1908 "build/C.tab.c"
    break;

  case 100: /* translation_unit: external_declaration  */
#line 241 "src/C.y"
                                        { yyval = yyvsp[0]; }
#line 1914 "build/C.tab.c"
    break;

  case 101: /* translation_unit: translation_unit external_declaration  */
#line 242 "src/C.y"
                                                { yyval = make_node('~', yyvsp[-1], yyvsp[0]);}
#line 1920 "build/C.tab.c"
    break;

  case 102: /* external_declaration: function_definition  */
#line 246 "src/C.y"
                                         { yyval = yyvsp[0]; }
#line 1926 "build/C.tab.c"
    break;

  case 103: /* external_declaration: declaration  */
#line 247 "src/C.y"
                                         { yyval = yyvsp[0]; }
#line 1932 "build/C.tab.c"
    break;

  case 104: /* function_definition: declaration_specifiers declarator declaration_list compound_statement  */
#line 251 "src/C.y"
                                                                                {
          yyval = make_node('D', make_node('d', yyvsp[-3], make_node('e', yyvsp[-2], yyvsp[-1])), yyvsp[0]); }
#line 1939 "build/C.tab.c"
    break;

  case 105: /* function_definition: declaration_specifiers declarator compound_statement  */
#line 253 "src/C.y"
                                                                {
          yyval = make_node('D', make_node('d', yyvsp[-2], yyvsp[-1]), yyvsp[0]); }
#line 1946 "build/C.tab.c"
    break;

  case 106: /* function_definition: declarator declaration_list compound_statement  */
#line 255 "src/C.y"
                                                          {
          yyval = make_node('D', make_node('d', yyvsp[-2], yyvsp[-1]), yyvsp[0]); }
#line 1953 "build/C.tab.c"
    break;

  case 107: /* function_definition: declarator compound_statement  */
#line 257 "src/C.y"
                                        { yyval = make_node('D', yyvsp[-1], yyvsp[0]); }
#line 1959 "build/C.tab.c"
    break;


#line 1963 "build/C.tab.c"

      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", YY_CAST (yysymbol_kind_t, yyr1[yyn]), &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;

  *++yyvsp = yyval;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
  {
    const int yylhs = yyr1[yyn] - YYNTOKENS;
    const int yyi = yypgoto[yylhs] + *yyssp;
    yystate = (0 <= yyi && yyi <= YYLAST && yycheck[yyi] == *yyssp
               ? yytable[yyi]
               : yydefgoto[yylhs]);
  }

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == YYEMPTY ? YYSYMBOL_YYEMPTY : YYTRANSLATE (yychar);
  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
      yyerror (YY_("syntax error"));
    }

  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */

      if (yychar <= YYEOF)
        {
          /* Return failure if at end of input.  */
          if (yychar == YYEOF)
            YYABORT;
        }
      else
        {
          yydestruct ("Error: discarding",
                      yytoken, &yylval);
          yychar = YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:
  /* Pacify compilers when the user code never invokes YYERROR and the
     label yyerrorlab therefore never appears in user code.  */
  if (0)
    YYERROR;

  /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  /* Pop stack until we find a state that shifts the error token.  */
  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
        {
          yyn += YYSYMBOL_YYerror;
          if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == YYSYMBOL_YYerror)
            {
              yyn = yytable[yyn];
              if (0 < yyn)
                break;
            }
        }

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
        YYABORT;


      yydestruct ("Error: popping",
                  YY_ACCESSING_SYMBOL (yystate), yyvsp);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END


  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", YY_ACCESSING_SYMBOL (yyn), yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturn;


/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturn;


#if !defined yyoverflow
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (YY_("memory exhausted"));
  yyresult = 2;
  goto yyreturn;
#endif


/*-------------------------------------------------------.
| yyreturn -- parsing is finished, clean up and return.  |
`-------------------------------------------------------*/
yyreturn:
  if (yychar != YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  YY_ACCESSING_SYMBOL (+*yyssp), yyvsp);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif

  return yyresult;
}

#line 259 "src/C.y"

#include <stdio.h>

extern char yytext[];
extern int column;

int yyerror(char *s)
{
	fflush(stdout);
	printf("\n%*s\n%*s\n", column, "^", column, s);
}

