
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14527(_: S4, _: S6, _: S4, _: S4, _: S5) {}
        
        fn test14527() { foo14527(S1, S2, S3, S4, S5, S6, S7, S8); }
    