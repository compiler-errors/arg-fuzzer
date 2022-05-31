
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11141(_: S0, _: S4, _: S6, _: S4, _: S7) {}
        
        fn test11141() { foo11141(S3, S4, S5, S6, S7, S8); }
    