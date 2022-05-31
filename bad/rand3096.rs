
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3096(_: S6, _: S4, _: S8) {}
        
        fn test3096() { foo3096(S1, S2, S3, S5, S6, S7, S8); }
    