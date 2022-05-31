
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8691(_: S2, _: S1, _: S5) {}
        
        fn test8691() { foo8691(S7, S4, S2, S0); }
    