
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16949(_: S2, _: S3, _: S4, _: S7) {}
        
        fn test16949() { foo16949(S5, S3, S2, S4, S1); }
    