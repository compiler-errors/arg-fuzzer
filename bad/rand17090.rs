
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo17090(_: S1, _: S2, _: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test17090() { foo17090(S6, S7, S6, S4, S2, S2); }
    