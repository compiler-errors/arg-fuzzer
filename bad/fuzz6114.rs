
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6114(_: S3, _: S5, _: S7) {}
        
        fn test6114() { foo6114(S1, S3, S4, S5, S6, S8); }
    