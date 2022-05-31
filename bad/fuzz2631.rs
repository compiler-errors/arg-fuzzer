
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2631(_: S1, _: S2, _: S5, _: S2, _: S5) {}
        
        fn test2631() { foo2631(S8, S6, S4, S2, S3, S5, S7, S1); }
    