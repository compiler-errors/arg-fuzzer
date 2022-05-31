
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1060(_: S8, _: S4) {}
        
        fn test1060() { foo1060(S6, S8, S3, S3, S8, S8); }
    