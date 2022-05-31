
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16865(_: S1, _: S3, _: S4, _: S5, _: S6, _: S7) {}
        
        fn test16865() { foo16865(S1, S2, S3, S4, S5, S6, S8); }
    