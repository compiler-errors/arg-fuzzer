
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6907(_: S2, _: S4, _: S8) {}
        
        fn test6907() { foo6907(S5, S6, S4, S7); }
    