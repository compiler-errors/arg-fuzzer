
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo855(_: S1, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test855() { foo855(S6, S1, S6, S8, S8, S4); }
    